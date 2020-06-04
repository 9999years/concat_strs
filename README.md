# concat_strs

[![docs.rs](https://docs.rs/concat_strs/badge.svg)](https://docs.rs/concat_strs/1.0.0/concat_strs/)
[![crates.io](https://img.shields.io/crates/v/concat_strs.svg)](https://crates.io/crates/concat_strs)
[![license: AGPL-3.0](https://img.shields.io/github/license/9999years/concat_strs)](https://www.gnu.org/licenses/agpl-3.0.en.html)
[![github.com/9999years/concat_strs](https://img.shields.io/badge/GitHub-9999years%2Fconcat__strs-blue)](https://github.com/9999years/concat_strs)

Provides the `concat_strs!` macro, which allows quickly building a `String`
from a number of components.

Example usage:

```rust
use concat_strs::concat_strs;

assert_eq!(
    "foo_bar_3.0",
    concat_strs!(
        "foo",
        '_',
        "bar",
        '_',
        3.0,
    )
);
```

This is [the fastest way to build a string from components][concat-benches].

[concat-benches]: https://github.com/hoodie/concatenation_benchmarks-rs