//!
//! main.rs
//!

use std::string::ToString;
use lazy_static::lazy_static;

use crate::selectors::Selectors;

mod article;
mod cli;
mod parse;
mod selectors;
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
    let body = url::get(APPLE_DEV_RELEASES.to_string()).unwrap();

    let articles = parse::parse_articles(body).unwrap();

    articles.iter().for_each(|article| {
        println!("{}", article);
    });
}
