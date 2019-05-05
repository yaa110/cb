#[macro_use]
extern crate log;
#[macro_use]
extern crate common;

use ctrlc;
use daemonize::Daemonize;
use gumdrop::Options;
use server::{setup_clipboard, AsyncUnix, Transmitter};
use std::fs;
use std::path::Path;

/// Crate version
const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Represents parsed options from command line
#[derive(Options)]
struct AppOptions {
    /// Prints the help message if it is `true`
    #[options(help = "Prints the help message", short = "h", long = "help")]
    pub help: bool,

    /// Prints the version if it is `true`
    #[options(help = "Prints the version", short = "V", long = "version")]
    pub version: bool,

    /// The app will run as a daemon if it is `true`
    #[options(help = "Runs the app as a daemon", short = "d", long = "daemon")]
    pub daemon: bool,
}

fn main() {
    let opts = AppOptions::parse_args_default_or_exit();

    env_logger::init();

    if let Err(e) = setup_clipboard() {
        fatal!("unable to setup clipboard: {}", e);
    }

    if opts.help {
        exit!("{}", AppOptions::usage());
    }

    if opts.version {
        exit!("{}", VERSION);
    }

    let socket_path = Path::new("/tmp/cb.sock");

    if opts.daemon {
        let daemonize = Daemonize::new().user("nobody").group("daemon").umask(0o000);

        if let Err(e) = daemonize.start() {
            fatal!("unable to run as daemon: {}", e);
        }
    }

    if let Err(e) = ctrlc::set_handler(move || {
        let _ = fs::remove_file(socket_path);
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
}
