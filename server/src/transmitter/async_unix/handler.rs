use super::codec::Codec;
use super::peer::Peer;
use futures::future::Future;
use std::error::Error;
use tokio::net::UnixStream;

/// Error handler of UNIX domain socket
pub(crate) fn on_error(e: impl Error) {
    error!("unable to accept a client: {}", e);
}

/// Client handler of UNIX domain socket
pub(crate) fn on_accept(client: UnixStream) {
    tokio::spawn(
        Peer::new(Codec::new(client)).map_err(|e| error!("unable to read from tunnel: {}", e)),
    );
}
