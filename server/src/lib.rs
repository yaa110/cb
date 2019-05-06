#[macro_use]
extern crate log;

mod clipboard_handler;
mod transmitter;

pub mod internal;

pub use transmitter::*;
