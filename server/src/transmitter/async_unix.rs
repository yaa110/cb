mod codec;
mod handler;
mod peer;

use super::Transmitter;
use common::errors::StringErrorResult;
use futures::stream::Stream;
use std::path::Path;
use tokio;
use tokio::net::UnixListener;

/// Represents an async unix domain socket transmitter
pub struct AsyncUnix {
    socket: UnixListener,
}

impl AsyncUnix {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        Ok(AsyncUnix {
            socket: UnixListener::bind(path).error_to_string()?,
        })
    }
}

impl Transmitter for AsyncUnix {
    fn listen(self) -> Result<(), String> {
        let server = self
            .socket
            .incoming()
            .map_err(handler::on_error)
            .for_each(move |client| {
                handler::on_accept(client);
                Ok(())
            });

        tokio::run(server);

        Ok(())
    }
}
