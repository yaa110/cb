mod async_unix;

pub use async_unix::*;

/// An interface to be implemented by transmitters
pub trait Transmitter {
    /// Starts listening
    fn listen(self) -> Result<(), String>;
}
