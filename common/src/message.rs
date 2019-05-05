mod action;
mod response;

pub use action::*;
pub use response::*;

/// The buffer size to read from socket
pub const BUFFER_SIZE: usize = 2048;

/// The splitter to frame byte stream
pub const SPLITTER: &[u8] = b"\r\n";

lazy_static! {
    /// The length of splitter
    pub static ref SPLITTER_LEN: usize = SPLITTER.len();
}
