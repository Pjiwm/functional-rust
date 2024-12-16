#![feature(fn_traits)]
#![feature(unboxed_closures)]

/// This macro creates a `ComposableFn` wrapper for a closure.
/// It takes a closure expression and wraps it into a `ComposableFn` type,
/// allowing you to compose the function with others using the `>>` operator.
///
/// ### Composition Operator
/// You can compose the function created by this macro with other functions using the
/// `>>` (right-to-left) operator, enabling easy chaining of transformations.
///
/// - **`>>` (Right-to-left composition)**:
///   The `>>` operator allows you to chain functions so that the output of the first function is passed as the input to the second function.
///   This creates a pipeline of transformations. In other words, `f1 >> f2` applies `f1` first, and then applies `f2` to the result of `f1`.
///   
///   Example:
///   ```rust
///   use functional_rs::{f, ComposableFn};
///   let add = f!(|x: i32| x + 1);
///   let multiply = f!(|x: i32| x * 2);
///   let composed = add >> multiply; // First add, then multiply
///   assert_eq!(composed(5), 12); // (5 + 1) * 2 = 12
///   ```
#[macro_export]
macro_rules! f {
    ($f:expr) => {
        ComposableFn(Box::new($f))
    };
}

/// This macro curries a function, allowing partial application of arguments.
/// It can handle various forms of argument types and function bodies.
///
/// ### Example
/// The `curry` macro allows you to create curried versions of functions, enabling partial application of arguments.
///
/// ```rust
/// // Import the `curry` macro
/// use functional_rs::c;
///
/// // Curry a simple function that adds two numbers
/// let add = c!(|a: i32, b: i32| a + b);
/// let add_5 = add(5); // Partially apply `5` to the function
/// assert_eq!(add_5(3), 8); // 5 + 3 = 8
/// ```
#[macro_export]
macro_rules! c (
    (|$first_arg:ident $(, $arg:ident )*| $function_body:expr) => {
       move |$first_arg| $(move |$arg|)* {
          $function_body
       }
    };
    (|$first_arg:ident:$first_arg_type:ty $(, $arg:ident:$arg_type:ty )*| $function_body:expr) => {
      move |$first_arg:$first_arg_type| $(move |$arg:$arg_type|)* {
         $function_body
      }
   };
   (|$first_arg:ident:$first_arg_type:ty $(, $arg:ident:$arg_type:ty )*| -> $ret_type:ty $function_body:block) => {
    move |$first_arg:$first_arg_type| $(move |$arg:$arg_type|)* -> $ret_type {
       $function_body
    };
   };
);

/// `ComposableFn` is a function wrapper that allows functions to be composed
/// using the `>>` operator. This enables chaining functions in a
/// readable manner, where functions can be combined to process data step by step.
/// It allows you to easily chain transformations or computations by passing the
/// output of one function to the input of the next.
pub struct ComposableFn<'a, T, U>(pub Box<dyn Fn(T) -> U + 'a>);

impl<'a, T, U> Fn<(T,)> for ComposableFn<'a, T, U>
where
    T: 'a,
    U: 'a,
{
    extern "rust-call" fn call(&self, args: (T,)) -> U {
        (self.0)(args.0)
    }
}

impl<'a, T, U> FnMut<(T,)> for ComposableFn<'a, T, U>
where
    T: 'a,
    U: 'a,
{
    extern "rust-call" fn call_mut(&mut self, args: (T,)) -> U {
        (self.0)(args.0)
    }
}

impl<'a, T, U> FnOnce<(T,)> for ComposableFn<'a, T, U>
where
    T: 'a,
    U: 'a,
{
    type Output = U;

    extern "rust-call" fn call_once(self, args: (T,)) -> U {
        (self.0)(args.0)
    }
}

impl<'a, T, U, V> std::ops::Shr<ComposableFn<'a, U, V>> for ComposableFn<'a, T, U>
where
    T: 'a,
    U: 'a,
    V: 'a,
{
    type Output = ComposableFn<'a, T, V>;

    fn shr(self, rhs: ComposableFn<'a, U, V>) -> Self::Output {
        ComposableFn(Box::new(move |x: T| rhs.0((self.0)(x))))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_num_parse_valid() {
        let from_str = i32::from_str;
        let parse_or_zero = |result: Result<i32, <i32 as FromStr>::Err>| result.unwrap_or(0);
        let num_parse = f!(from_str) >> f!(parse_or_zero);

        assert_eq!(num_parse("10"), 10);
    }

    #[test]
    fn test_num_parse_invalid() {
        let from_str = i32::from_str;
        let parse_or_zero = |result: Result<i32, <i32 as FromStr>::Err>| result.unwrap_or(0);
        let num_parse = f!(from_str) >> f!(parse_or_zero);

        assert_eq!(num_parse("THIS IS NOT A NUMBER"), 0);
    }

    #[test]
    fn test_first_word_parse_valid() {
        let first_word = f!(|s: &str| s.split_whitespace().next().unwrap_or(""));
        let from_str = i32::from_str;
        let parse_or_zero = |result: Result<i32, <i32 as FromStr>::Err>| result.unwrap_or(0);
        let first_word_parse = first_word >> f!(from_str) >> f!(parse_or_zero);

        assert_eq!(first_word_parse("100 THIS IS A NUMBER"), 100);
    }

    #[test]
    fn test_first_word_parse_invalid() {
        let first_word = f!(|s: &str| s.split_whitespace().next().unwrap_or(""));
        let from_str = i32::from_str;
        let parse_or_zero = |result: Result<i32, <i32 as FromStr>::Err>| result.unwrap_or(0);
        let first_word_parse = first_word >> f!(from_str) >> f!(parse_or_zero);

        assert_eq!(first_word_parse("100THIS IS NOT A NUMBER"), 0);
    }

    #[test]
    fn test_add_10_from_str() {
        let from_str = i32::from_str;
        let parse_or_zero = |result: Result<i32, <i32 as FromStr>::Err>| result.unwrap_or(0);
        let add = c!(|a: i32, b: i32| a + b);
        let add_10_from_str = f!(from_str) >> f!(parse_or_zero) >> f!(add(10));

        assert_eq!(add_10_from_str("4"), 14);
    }
}
