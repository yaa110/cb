/// Logs an error message and terminates the process by exit status code of 1
///
/// # Examples
///
/// ```rust,ignore
/// fatal!("An error message: {}", "a general error");
/// ```
#[macro_export]
macro_rules! fatal {
    ($($arg:tt)*) => {{
        error!($($arg)*);
        std::process::exit(1);
    }}
}
