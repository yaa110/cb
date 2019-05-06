/// The UNIX domain socket path
pub const SOCKET_PATH: &str = "/tmp/cb.sock";

/// The buffer size to read from socket
pub const BUFFER_SIZE: usize = 2048;

/// The splitter to frame byte stream
pub const SPLITTER: &[u8] = b"\r\n";

lazy_static! {
    /// The length of splitter
    pub static ref SPLITTER_LEN: usize = SPLITTER.len();
}
