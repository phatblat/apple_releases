//!
//! cli.rs
//!

use clap::{Arg, ArgAction, Command};

/// Parses command line arguments.
pub(crate) fn cli() -> Command {
    Command::new("apple_releases")
        .about("CLI for the Apple Developer News RSS feed")
        .version("0.1.0")
        .author("Ben Chatelain")
        .arg(
            // --all
            Arg::new("all")
                .long("all")
                .short('a')
                .help("Show all releases")
                .action(ArgAction::SetTrue),
        )
        .arg(
            // --unfurl
            Arg::new("unfurl")
                .long("unfurl")
                .short('u')
                .help("Unfurl release note URLs")
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
