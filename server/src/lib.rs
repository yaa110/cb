#[macro_use]
extern crate log;
#[macro_use]
extern crate common;

mod clipboard_handler;
mod transmitter;

pub mod internal;

pub use transmitter::*;

use common::constants::SOCKET_PATH;
use ctrlc;
use daemonize::Daemonize;
use internal::get_user_group;
use std::fs;
use std::path::Path;
use transmitter::{AsyncUnix, Transmitter};

/// Removes created files
fn clean() {
    let _ = fs::remove_file(Path::new(SOCKET_PATH));
}

/// Starts server as a daemon
pub fn start() {
    env_logger::init();

    let socket_path = Path::new(SOCKET_PATH);

    let (username, group) = match get_user_group() {
        Ok(ug) => ug,
        Err(e) => {
            fatal!("{}", e);
        }
    };

    let daemonize = Daemonize::new()
        .user(username.as_str())
        .group(group.as_str())
        .umask(0o000);

    if let Err(e) = daemonize.start() {
        fatal!("unable to run as daemon: {}", e);
    }

    if let Err(e) = ctrlc::set_handler(move || {
        clean();
        std::process::exit(0);
    }) {
        fatal!("unable to set handler of CTRL-C signals: {}", e);
    }

    if socket_path.exists() {
        if let Err(e) = fs::remove_file(socket_path) {
            fatal!("unable to delete UNIX domain socket file: {}", e);
        }
    }

    if let Err(e) = AsyncUnix::new(socket_path).and_then(Transmitter::listen) {
        fatal!("unable to start transmitter: {}", e);
    }

    clean();
}
