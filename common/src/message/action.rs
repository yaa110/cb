use crate::errors::StringErrorResult;
use bincode::{deserialize, serialize};
use bytes::BytesMut;
use std::convert::TryFrom;
use std::convert::TryInto;

/// Represents variant types of content action
#[derive(Serialize, Deserialize)]
pub enum Action {
    /// An action to clear clipboard
    Clear,

    /// An action to get content from clipboard
    Get,

    /// An action to set a content to clipboard
    Set(String),
}

impl TryInto<BytesMut> for Action {
    type Error = String;

    fn try_into(self) -> Result<BytesMut, Self::Error> {
        Ok(BytesMut::from(serialize(&self).error_to_string()?))
    }
}

impl TryFrom<BytesMut> for Action {
    type Error = String;

    fn try_from(value: BytesMut) -> Result<Self, Self::Error> {
        Ok(deserialize(&value).error_to_string()?)
    }
}
