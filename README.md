# concat_strs

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