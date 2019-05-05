use crate::errors::StringErrorResult;
use bincode::{deserialize, serialize};
use bytes::BytesMut;
use std::convert::TryFrom;
use std::convert::TryInto;

/// Represents the response of clipboard action,
#[derive(Serialize, Deserialize)]
pub struct Response {
    /// A `true` value represents a successful action
    pub status: bool,

    /// A `None` value is used for set and clear actions
    pub content: Option<String>,
}

impl TryInto<BytesMut> for Response {
    type Error = String;

    fn try_into(self) -> Result<BytesMut, Self::Error> {
        Ok(BytesMut::from(serialize(&self).error_to_string()?))
    }
}

impl TryFrom<BytesMut> for Response {
    type Error = String;

    fn try_from(value: BytesMut) -> Result<Self, Self::Error> {
        Ok(deserialize(&value).error_to_string()?)
    }
}
