//! Provides the `concat_strs!` macro, which allows quickly building a `String`
//! from a number of components.
//!
//! Example usage:
//!
//! ```
//! use concat_strs::concat_strs;
//!
//! assert_eq!(
//!     "foo_bar_3.0",
//!     concat_strs!(
//!         "foo",
//!         '_',
//!         "bar",
//!         '_',
//!         3.0,
//!     )
//! );
//! ```

use proc_macro_hack::proc_macro_hack;

/// Concatenates a number of `&str` expressions, `&str` literals, and `char`
/// literals together.
///
/// Generated code for
/// ```
/// use concat_strs::concat_strs;
///
/// let bar = "some &str expression";
/// let s = concat_strs!(
///     "foo",
///     ' ',
///     bar,
///     'c',
///     "baz",
/// );
/// assert_eq!(s, "foo some &str expressioncbaz");
/// ```
/// is roughly
/// ```
/// let bar = "some &str expression";
/// let s = {
///     let tmp1 = bar;
///     let tmp1_len = bar.len();
///     let mut ret = String::with_capacity(tmp1_len + 8);
///     ret.push_str("foo");
///     ret.push(' ');
///     ret.push_str(tmp1);
///     ret.push('c');
///     ret.push_str("baz");
///     ret
/// };
/// assert_eq!(s, "foo some &str expressioncbaz");
/// ```
///
/// This is [the fastest way to build a string from components][concat-benches].
///
/// [concat-benches]: https://github.com/hoodie/concatenation_benchmarks-rs
#[proc_macro_hack]
pub use concat_strs_impl::concat_strs;
