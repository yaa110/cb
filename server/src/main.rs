#[macro_use]
extern crate log;
#[macro_use]
extern crate common;

use common::constants::{SERVER_PID, SOCKET_PATH};
use ctrlc;
use daemonize::Daemonize;
use server::internal::get_user_group;
use server::{setup_clipboard, AsyncUnix, Transmitter};
use std::fs;
use std::path::Path;

/// Removes created files
fn clean() {
    let _ = fs::remove_file(Path::new(SOCKET_PATH));
    let _ = fs::remove_file(Path::new(SERVER_PID));
}

fn main() {
    env_logger::init();

    if let Err(e) = setup_clipboard() {
        fatal!("unable to setup clipboard: {}", e);
    }

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
        .pid_file(SERVER_PID)
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
