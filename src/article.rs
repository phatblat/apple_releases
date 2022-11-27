//!
//! article.rs
//!

use std::fmt::{Display, Formatter};
use url::Url;

/// An article from the Apple Developer software releases site.
pub struct Article {
    pub title: String,
    pub date: String,
    pub release_notes_url: Option<Url>,
}

impl Display for Article {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let url = self.release_notes_url.as_ref().map_or(None, |url| Some(url.to_string()));
        write!(f, "{} - {}, <{}>", self.date, self.title, url.unwrap_or_default())
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[test]
fn test_article_display() {
    let article = Article {
        title: "iOS 16.2 beta 3 (20C5049e)".to_string(),
        date: "November 15, 2022".to_string(),
        release_notes_url: Url::parse("https://developer.apple.com/go/?id=ios-16.2-rn").ok(),
    };

    assert_eq!(
        article.to_string(),
        "November 15, 2022 - iOS 16.2 beta 3 (20C5049e), <https://developer.apple.com/go/?id=ios-16.2-rn>"
    );
}

#[test]
fn test_article_display_without_url() {
    let article = Article {
        title: "App Store Connect 1.11".to_string(),
        date: "November 15, 2022".to_string(),
        release_notes_url: None,
    };

    assert_eq!(
        article.to_string(),
        "November 15, 2022 - App Store Connect 1.11, <>"
    );
}