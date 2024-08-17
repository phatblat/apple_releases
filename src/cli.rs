//!
//! cli.rs
//!

use clap::{Arg, ArgAction, Command};

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Parses command line arguments.
pub(crate) fn cli() -> Command {
    Command::new("apple_releases")
        .about("CLI for the Apple Developer News RSS feed")
        .version(VERSION)
        .author("Ben Chatelain")
        .arg(
            // --all
            Arg::new("all")
                .long("all")
                .short('a')
                .help("Show all releases")
                .action(ArgAction::SetTrue),
        )
        .after_help(
            "This tool parses the content of the Apple Developer Software Updates page: \
            https://developer.apple.com/news/releases/",
        )
}

/* ---------------------------------------------------------------------------------------------- */

#[test]
fn verify_cli() {
    cli().debug_assert();
}
