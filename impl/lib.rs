//! Implementation crate for `concat_strs`, needed for [`proc_macro_hack`][proc_macro_hack]
//!
//! [proc_macro_hack]: https://docs.rs/proc-macro-hack/

use proc_macro as pm;
use proc_macro2::{Span, TokenStream};
use proc_macro_hack::proc_macro_hack;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token;

/// `concat_strs` macro; see documentation for the `concat_strs` crate for more information.
#[proc_macro_hack]
pub fn concat_strs(input: pm::TokenStream) -> pm::TokenStream {
    let macro_input = syn::parse_macro_input!(input as ConcatStrs);
    impl_concat_strs(macro_input).into()
}

struct ConcatStrs {
    exprs: Punctuated<syn::Expr, token::Comma>,
}

impl Parse for ConcatStrs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            exprs: Punctuated::parse_terminated(input)?,
        })
    }
}

impl ConcatStrs {
    fn bytes_needed(&self) -> usize {
        self.exprs.iter().map(|e| byte_count(e)).sum()
    }
}

fn byte_count(expr: &syn::Expr) -> usize {
    match expr {
        syn::Expr::Lit(lit) => match &lit.lit {
            syn::Lit::Str(lit) => lit.value().len(),
            syn::Lit::Char(lit) => lit.value().len_utf8(),
            syn::Lit::Int(_) | syn::Lit::Float(_) => lit.lit.to_token_stream().to_string().len(),
            _ => 0,
        },
        _ => 0,
    }
}

fn tmp_assign_tokens(
    expr: &syn::Expr,
    tmp_ident: &syn::Ident,
) -> Option<(syn::Ident, TokenStream)> {
    match expr {
        syn::Expr::Lit(_) => None,
        _ => {
            let len_ident = syn::Ident::new(&format!("{}_len", tmp_ident), Span::call_site());
            Some((
                len_ident.clone(),
                quote! {
                    let #tmp_ident = #expr;
                    let #len_ident = #tmp_ident.len();
                },
            ))
        }
    }
}

fn push_tokens(expr: &syn::Expr, tmp_ident: &syn::Ident, string_ident: &syn::Ident) -> TokenStream {
    match expr {
        syn::Expr::Lit(expr) => match expr.lit {
            syn::Lit::Str(_) => {
                quote! {
                    #string_ident.push_str(#expr);
                }
            }
            syn::Lit::Char(_) => {
                quote! {
                    #string_ident.push(#expr);
                }
            }
            syn::Lit::Float(_) | syn::Lit::Int(_) => {
                let expr_string =
                    syn::LitStr::new(&expr.lit.to_token_stream().to_string(), Span::call_site());
                quote! {
                    #string_ident.push_str(#expr_string);
                }
            }

            _ => panic!("Unsupported expression {}", expr.to_token_stream()),
        },
        _ => {
            quote! {
                #string_ident.push_str(#tmp_ident);
            }
        }
    }
}

fn tmp_idents() -> impl Iterator<Item = syn::Ident> {
    (0..)
        .into_iter()
        .map(|n| syn::Ident::new(&format!("__tmp_{}", n), Span::call_site()))
}

fn impl_concat_strs(items: ConcatStrs) -> TokenStream {
    let bytes = items.bytes_needed();
    let string_ident: syn::Ident = syn::Ident::new("__ret", Span::call_site());

    let tmp_assignments_idents = items
        .exprs
        .iter()
        .zip(tmp_idents())
        .map(|(expr, ident)| (tmp_assign_tokens(expr, &ident), ident))
        .collect::<Vec<_>>();

    let tmp_len_idents = tmp_assignments_idents
        .iter()
        .filter_map(|(assign, _ident)| {
            if let Some((len_ident, _assign)) = assign {
                Some(len_ident)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let tmp_assignments = tmp_assignments_idents
        .iter()
        .filter_map(|(assign, _ident)| assign.clone().map(|(_len_ident, assign)| assign))
        .collect::<Vec<_>>();

    let push_exprs = items
        .exprs
        .iter()
        .zip(&tmp_assignments_idents)
        .map(|(expr, (_expr, tmp_ident))| push_tokens(expr, &tmp_ident, &string_ident))
        .collect::<Vec<_>>();

    quote! {{
        #(#tmp_assignments)*
        let mut #string_ident = String::with_capacity(#(#tmp_len_idents +)* #bytes);
        #(#push_exprs)*
        #string_ident
    }}
}
