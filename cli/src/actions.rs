use bytes::buf::BufMut;
use bytes::BytesMut;
use common::constants::{SOCKET_PATH, SPLITTER, SPLITTER_LEN};
use common::errors::StringErrorResult;
use common::message::{Action, Response};
use hex;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;

/// Sends action to server and receives the response
fn send_action(action: Action) -> Result<Response, String> {
    let mut stream = UnixStream::connect(SOCKET_PATH).error_to_string()?;
    let buf: BytesMut = action.try_into()?;
    let hex_buf = hex::encode(&buf).into_bytes();
    let mut data = BytesMut::with_capacity(hex_buf.len() + *SPLITTER_LEN);
    data.put(hex_buf);
    data.put(SPLITTER);
    stream.write_all(&data).error_to_string()?;
    let mut res = Vec::new();
    stream.read_to_end(&mut res).error_to_string()?;
    Response::try_from(BytesMut::from(
        hex::decode(&res[..res.len() - *SPLITTER_LEN]).error_to_string()?,
    ))
}

/// Stores `content` to clipboard
pub fn set(content: Option<String>) -> bool {
    send_action(
        content
            .and_then(|content| Some(Action::Set(content)))
            .unwrap_or(Action::Clear),
    )
    .ok()
    .and_then(|res| Some(res.status))
    .unwrap_or(false)
}

/// Retrieves the content of clipboard
pub fn get() -> Option<String> {
    let res = send_action(Action::Get).ok()?;
    if res.status {
        res.content
    } else {
        None
    }
}

/// Clears the content of clipboard
pub fn clear() -> bool {
    set(None)
}
