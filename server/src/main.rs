#[macro_use]
extern crate gumdrop;

use gumdrop::Options;
use std::process::exit;

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

    /// The path of unix domain socket, the default value is `./cb.sock`
    #[options(
        help = "Sets the path of unix domain socket, the default value is `./cb.sock`",
        short = "s",
        long = "socket",
        meta = "PATH"
    )]
    pub socket: Option<String>,

    /// The app will run as a daemon if it is `true`
    #[options(help = "Runs the app as a daemon", short = "d", long = "daemon")]
    pub daemon: bool,
}

/// Prints a message to stdout and exits with the status code of `0`
fn print_and_exit(message: &str) {
    println!("{}", message);
    exit(0);
}

fn main() {
    let opts = AppOptions::parse_args_default_or_exit();

    if opts.help {
        print_and_exit(AppOptions::usage());
    }

    if opts.version {
        print_and_exit(VERSION);
    }
}
