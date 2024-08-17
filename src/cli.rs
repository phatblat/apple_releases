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
            "Longer explanation to appear after the options when \
                 displaying the help information from --help or -h",
        )
}

/* ---------------------------------------------------------------------------------------------- */

#[test]
fn verify_cli() {
    cli().debug_assert();
}
