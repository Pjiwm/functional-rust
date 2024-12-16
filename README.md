
# functional_rs
`functional_rs` is an experimental crate that brings functional programming concepts to Rust using Nightly features. By implementing `Fn` on structs, it enables function composition, currying, and other higher-order functional programming patterns. The goal is to keep the syntax as concise as possible, making functional patterns easy to use without cluttering the code.

This crate is designed for **Nightly Rust** and uses experimental features, so it’s not stable yet.

## Example

```rust
use functional_rs::{c, f, ComposableFn};
use std::str::FromStr;

fn main() {
    let from_str = i32::from_str;
    let parse_or_zero = |result: Result<i32, <i32 as FromStr>::Err>| result.unwrap_or(0);

    // Currying with composition
    let add = c!(|a: i32, b: i32| a + b);
    let add_10_from_str = f!(from_str) >> f!(parse_or_zero) >> f!(add(10));

    // Using the composed function
    assert_eq!(add_10_from_str("4"), 14); // (4 -> parse -> add 10) = 14
    assert_eq!(add_10_from_str("not a number"), 10); // invalid input -> default 10
}
