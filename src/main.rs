//!
//! main.rs
//!

use crate::cli::cli;
use crate::selectors::Selectors;
use lazy_static::lazy_static;
use std::string::ToString;

mod article;
mod cli;
mod parse;
mod product;
mod selectors;
mod software_release;
mod url;

/* ---------------------------------------------------------------------------------------------- */

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

/// User agent for network requests.
const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// Webpage listing new software releases.
const APPLE_DEV_RELEASES: &str = "https://developer.apple.com/news/releases/";

lazy_static! {
    static ref SELECTORS: Selectors = Selectors::new();
}

/* ---------------------------------------------------------------------------------------------- */

/// Executable entry point.
fn main() {
    let args = cli().get_matches();
    let show_all = args.get_one::<bool>("all").unwrap();
    let unfurl_urls = args.get_one::<bool>("unfurl").unwrap();

    let body = url::get(APPLE_DEV_RELEASES.to_string()).unwrap();

    let articles = parse::parse_articles(body).unwrap();

    articles
        .iter()
        .for_each(|article| match article.software_release {
            Some(_) => {
                println!("{}", article);
                // if *unfurl_urls {
                //     println!(" - {}", article.release_notes_url_unfurled().unwrap());
                // } else {
                //     println!(" - {}", article.release_notes_url.as_ref().unwrap());
                // }
            }
            _ => {
                if *show_all {
                    println!("{}", article);
                }
            }
        });
}
