use std::error::Error;

/// An interface to convert Result<T, E> to Result<T, String>
pub trait StringErrorResult<T, E: Error> {
    /// Converts Result<T, E> to Result<T, String>
    fn error_to_string(self) -> Result<T, String>;
}

impl<T, E: Error> StringErrorResult<T, E> for Result<T, E> {
    fn error_to_string(self) -> Result<T, String> {
        self.map_err(|e| e.description().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Error, ErrorKind};

    #[test]
    fn test_error_to_string() {
        let res: Result<(), Error> = Err(Error::new(ErrorKind::Other, "a custom error"));
        if let Err(e) = res.error_to_string() {
            assert_eq!(e, "a custom error");
        } else {
            panic!("An error is expected");
        }
    }
}
