#[macro_use]
extern crate common;

/// Crate version
const VERSION: &str = env!("CARGO_PKG_VERSION");

use gumdrop::Options;

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
}

fn main() {
    let opts = AppOptions::parse_args_default_or_exit();

    if opts.help {
        exit!("{}", AppOptions::usage());
    }

    if opts.version {
        exit!("{}", VERSION);
    }

    if opts.paste {
        match cli::get() {
            Some(content) => {
                exit!("{}", content);
            }
            None => {
                oops!("An error occurred");
            }
        }
    }

    // TODO
}
