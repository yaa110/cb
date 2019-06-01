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
use internal::UserGroup;
use std::fs;
use std::path::Path;
use transmitter::{AsyncUnix, Transmitter};

/// Removes created files
fn clean_and_exit() {
    let _ = fs::remove_file(Path::new(SOCKET_PATH));
    std::process::exit(0)
}

/// Starts server as a daemon
pub fn start() {
    env_logger::init();

    let socket_path = Path::new(SOCKET_PATH);

    let user_group = match UserGroup::get() {
        Ok(ug) => ug,
        Err(e) => {
            fatal!("{}", e);
        }
    };

    let daemonize = Daemonize::new()
        .user(user_group.user.as_str())
        .group(user_group.group.as_str())
        .umask(0o000);

    if let Err(e) = daemonize.start() {
        fatal!("unable to run as daemon: {}", e);
    }
    // FIXME Server is daemon process how we can send signal SIGINT ( CTRL+C ) ?
    if let Err(e) = ctrlc::set_handler(clean_and_exit) {
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
    clean_and_exit();
}
