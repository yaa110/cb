/// Prints a message to stderr and terminates the process by exit status code of 1
///
/// # Examples
///
/// ```rust,ignore
/// oops!("A message: {}", "a general text");
/// ```
#[macro_export]
macro_rules! oops {
    ($($arg:tt)*) => {{
        eprintln!($($arg)*);
        std::process::exit(1);
    }}
}
