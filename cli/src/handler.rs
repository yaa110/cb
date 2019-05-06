use bytes::buf::BufMut;
use bytes::BytesMut;
use common::constants::{SPLITTER, SPLITTER_LEN};
use common::errors::StringErrorResult;
use common::message::{Action, Response};
use hex;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::{Read, Write};

/// Represents clipboard handler
pub struct Handler<T: Read + Write> {
    stream: T,
}

impl<T: Read + Write> Handler<T> {
    pub fn new(stream: T) -> Self {
        Handler { stream }
    }

    /// Sends action to server and receives the response
    fn send_action(&mut self, action: Action) -> Result<Response, String> {
        let buf: BytesMut = action.try_into()?;
        let hex_buf = hex::encode(&buf).into_bytes();
        let mut data = BytesMut::with_capacity(hex_buf.len() + *SPLITTER_LEN);
        data.put(hex_buf);
        data.put(SPLITTER);
        self.stream.write_all(&data).error_to_string()?;
        let mut res = Vec::new();
        self.stream.read_to_end(&mut res).error_to_string()?;
        Response::try_from(BytesMut::from(
            hex::decode(&res[..res.len() - *SPLITTER_LEN]).error_to_string()?,
        ))
    }

    /// Stores `content` to clipboard
    pub fn set(&mut self, content: Option<String>) -> bool {
        self.send_action(
            content
                .and_then(|content| Some(Action::Set(content)))
                .unwrap_or(Action::Clear),
        )
        .ok()
        .and_then(|res| Some(res.status))
        .unwrap_or(false)
    }

    /// Retrieves the content of clipboard
    pub fn get(&mut self) -> Option<String> {
        let res = self.send_action(Action::Get).ok()?;
        if res.status {
            res.content
        } else {
            None
        }
    }

    /// Clears the content of clipboard
    pub fn clear(&mut self) -> bool {
        self.set(None)
    }
}
