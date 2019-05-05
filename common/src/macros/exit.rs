/// Prints a message to stdout and terminates the process by exit status code of 0
///
/// # Examples
///
/// ```rust,ignore
/// exit!("A message: {}", "a general text");
/// ```
#[macro_export]
macro_rules! exit {
    ($($arg:tt)*) => {
        println!($($arg)*);
        std::process::exit(0);
    }
}
