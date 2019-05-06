#[macro_use]
extern crate common;

/// Crate version
const VERSION: &str = env!("CARGO_PKG_VERSION");

use cli::Handler;
use common::constants::SOCKET_PATH;
use common::errors::StringErrorResult;
use gumdrop::Options;
use std::io::{self, Read};
use std::os::unix::net::UnixStream;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

/// Represents parsed options from command line
#[derive(Options)]
struct AppOptions {
    /// Prints the help message if it is `true`
    #[options(help = "Prints the help message", short = "h", long = "help")]
    pub help: bool,

    /// Prints the version if it is `true`
    #[options(help = "Prints the version", short = "V", long = "version")]
    pub version: bool,

    /// Pastes the content of clipboard
    #[options(help = "Pastes the content of clipboard", short = "p", long = "paste")]
    pub paste: bool,

    /// Clears the content of clipboard
    #[options(help = "Clears the content of clipboard", short = "c", long = "clear")]
    pub clear: bool,

    /// Do not print newline after pasting the content
    #[options(
        help = "Do not print newline after pasting the content",
        short = "r",
        long = "raw"
    )]
    pub raw: bool,

    /// Stores `text` into clipboard
    #[options(
        help = "Store TEXT into clipboard",
        meta = "TEXT",
        short = "t",
        long = "text"
    )]
    pub text: Option<String>,
}

/// Tries to connect to server
fn try_connect(try_run: bool) -> Result<UnixStream, String> {
    match UnixStream::connect(SOCKET_PATH) {
        Ok(stream) => Ok(stream),
        err => {
            if try_run {
                let _ = Command::new("cbs")
                    .spawn()
                    .error_to_string()?
                    .wait()
                    .error_to_string()?;
                // FIXME: find a way to remove sleep and ensure that `cbs` is running
                sleep(Duration::from_secs(1));
                try_connect(false)
            } else {
                err.error_to_string()
            }
        }
    }
}

fn main() {
    let mut opts = AppOptions::parse_args_default_or_exit();

    if opts.help {
        exit!("{}", AppOptions::usage());
    }

    if opts.version {
        exit!("{}", VERSION);
    }

    let mut handler = Handler::new(match try_connect(true) {
        Ok(stream) => stream,
        Err(e) => {
            oops!("[error] unable to connect to server: {}", e);
        }
    });

    if opts.clear {
        if handler.clear() {
            std::process::exit(0);
        } else {
            oops!("[error] an error occurred");
        }
    }

    if opts.paste {
        match handler.get() {
            Some(content) => {
                if opts.raw {
                    print!("{}", content);
                    std::process::exit(0);
                } else {
                    exit!("{}", content);
                }
            }
            None => {
                oops!("[error] an error occurred");
            }
        }
    }

    if opts.text.is_some() {
        if handler.set(opts.text.take()) {
            std::process::exit(0);
        } else {
            oops!("[error] an error occurred");
        }
    }

    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    if let Err(e) = handle.read_to_string(&mut buffer) {
        oops!("[error] unable to read piped text: {}", e);
    }

    if handler.set(Some(buffer)) {
        std::process::exit(0);
    } else {
        oops!("[error] an error occurred");
    }
}
