//! 本模块用于处理一些复杂计算

/// # Panics
///
/// The function panics if the second argument is zero.
///
/// ```rust,should_panic
/// // panics on division by zero
/// doc_comments::compute::div(10, 0);
/// ```
#[doc(hidden)]
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }

    a / b
}

/// ```
/// # fn try_main() -> Result<(), String> {
/// let res = doc_comments::compute::try_div(10, 1)?;
/// # Ok(()) // returning from try_main
/// # }
/// # fn main() {
/// #    try_main().unwrap();
/// #
/// # }
/// ```
#[doc(hidden)]
pub fn try_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Divide-by-zero"))
    } else {
        Ok(a / b)
    }
}
