use bytes::buf::BufMut;
use bytes::BytesMut;
use common::constants::{BUFFER_SIZE, SPLITTER, SPLITTER_LEN};
use hex;
use std::error::Error;
use tokio::net::UnixStream;
use tokio::prelude::{Async, AsyncRead, AsyncWrite, Poll, Stream};

/// Represents a UNIX socket poller to send and receive data by splitter
#[derive(Debug)]
pub(crate) struct Codec {
    /// The UNIX socket to send and receive data
    socket: UnixStream,

    /// The buffer for reading data
    read_buf: BytesMut,

    /// The buffer for writing data
    write_buf: BytesMut,
}

impl Codec {
    /// Creates a new instance of `Codec`
    pub fn new(stream: UnixStream) -> Self {
        Codec {
            socket: stream,
            read_buf: BytesMut::new(),
            write_buf: BytesMut::new(),
        }
    }

    /// Reads data from socket
    fn read_data(&mut self) -> Poll<(), String> {
        loop {
            self.read_buf.reserve(BUFFER_SIZE);
            let n = match self.socket.read_buf(&mut self.read_buf) {
                Ok(Async::Ready(t)) => t,
                Ok(Async::NotReady) => return Ok(Async::NotReady),
                Err(e) => return Err(e.description().to_string()),
            };
            if n == 0 {
                return Ok(Async::Ready(()));
            }
        }
    }

    /// Writes `data` to write buffer
    pub fn buffer(&mut self, data: BytesMut) {
        let hex_buf = hex::encode(&data).into_bytes();
        self.write_buf.reserve(hex_buf.len());
        self.write_buf.put(hex_buf);
        self.write_buf.extend_from_slice(SPLITTER);
    }

    /// Flushes write buffer to the socket
    pub fn poll_flush(&mut self) -> Poll<(), String> {
        while !self.write_buf.is_empty() {
            let size = match self.socket.poll_write(&self.write_buf) {
                Ok(Async::Ready(t)) => t,
                Ok(Async::NotReady) => return Ok(Async::NotReady),
                Err(e) => return Err(e.description().to_string()),
            };
            if size == 0 {
                return Err(String::from("writing zero bytes to the socket"));
            }
            let _ = self.write_buf.split_to(size);
        }
        Ok(Async::Ready(()))
    }
}

impl Stream for Codec {
    type Item = BytesMut;
    type Error = String;

    fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {
        let is_closed = self.read_data()?.is_ready();
        let splitter_pos = self
            .read_buf
            .windows(*SPLITTER_LEN)
            .enumerate()
            .find(|&(_, b)| b == SPLITTER)
            .map(|(i, _)| i);
        if let Some(pos) = splitter_pos {
            let mut data = self.read_buf.split_to(pos + *SPLITTER_LEN);
            let _ = data.split_off(pos);
            if let Ok(hex_decoded) = hex::decode(&data) {
                return Ok(Async::Ready(Some(BytesMut::from(hex_decoded))));
            }
        }
        if is_closed {
            Ok(Async::Ready(None))
        } else {
            Ok(Async::NotReady)
        }
    }
}
