use bytes::BytesMut;
use clipboard::{ClipboardContext, ClipboardProvider};
use common::errors::StringErrorResult;
use common::message::{Action, Response};
use std::convert::TryFrom;
use std::convert::TryInto;
use std::sync::RwLock;

static mut CLIPBOARD: Option<RwLock<ClipboardContext>> = None;

pub fn setup_clipboard() -> Result<(), String> {
    unsafe {
        CLIPBOARD = Some(RwLock::new(
            ClipboardProvider::new().map_err(|e| e.description().to_string())?,
        ));
    }
    Ok(())
}

fn handle_action_by_error(data: BytesMut) -> Result<Response, String> {
    let mut ctx = unsafe { CLIPBOARD.as_ref().unwrap().write().error_to_string()? };
    let content = match Action::try_from(data)? {
        Action::Clear => {
            ctx.set_contents(String::new())
                .map_err(|e| e.description().to_string())?;
            None
        }
        Action::Get => Some(
            ctx.get_contents()
                .map_err(|e| e.description().to_string())?,
        ),
        Action::Set(msg) => {
            ctx.set_contents(msg)
                .map_err(|e| e.description().to_string())?;
            None
        }
    };
    Ok(Response {
        status: true,
        content,
    })
}

/// Handles actions and returns the response of action
pub fn handle_action(data: BytesMut) -> Result<BytesMut, String> {
    match handle_action_by_error(data) {
        Ok(res) => res,
        Err(e) => {
            error!("unable to perform action: {}", e);
            Response {
                status: false,
                content: None,
            }
        }
    }
    .try_into()
}
