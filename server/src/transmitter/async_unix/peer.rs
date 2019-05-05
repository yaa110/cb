use super::codec::Codec;
use crate::clipboard_handler::handle_action;
use futures::future::Future;
use futures::stream::Stream;
use futures::Poll;
use tokio::prelude::Async;

/// Represents the state for the connected client
pub(crate) struct Peer {
    /// The UNIX domain socket to send and receive data
    codec: Codec,
}

impl Peer {
    /// Creates a new instance of `Peer`
    pub fn new(codec: Codec) -> Self {
        Peer { codec }
    }
}

impl Future for Peer {
    type Item = ();
    type Error = String;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        while let Async::Ready(data) = self.codec.poll()? {
            if let Some(data) = data {
                self.codec.buffer(handle_action(data).unwrap_or_default());
            } else {
                // EOF has been received
                return Ok(Async::Ready(()));
            }
        }
        let _ = self.codec.poll_flush()?;
        Ok(Async::NotReady)
    }
}
