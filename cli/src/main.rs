#[macro_use]
extern crate common;

/// Crate version
const VERSION: &str = env!("CARGO_PKG_VERSION");

use cli::Handler;
use common::constants::SOCKET_PATH;
use common::errors::StringErrorResult;
use gumdrop::Options;
use std::os::unix::net::UnixStream;
use std::process::Command;

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
}

/// Tries to connect to server
fn try_connect(try_run: bool) -> Result<UnixStream, String> {
    match UnixStream::connect(SOCKET_PATH) {
        Ok(stream) => Ok(stream),
        err => {
            if try_run {
                let _ = Command::new("cbs").spawn().error_to_string()?;
                try_connect(false)
            } else {
                err.error_to_string()
            }
        }
    }
}

fn main() {
    let opts = AppOptions::parse_args_default_or_exit();

    if opts.help {
        exit!("{}", AppOptions::usage());
    }

    if opts.version {
        exit!("{}", VERSION);
    }

    let mut handler = Handler::new(if let Ok(stream) = try_connect(true) {
        stream
    } else {
        oops!("[error] unable to connect to server");
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

    // TODO read content from stdin and set content
}
