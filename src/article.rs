//!
//! article.rs
//!

use std::fmt::{Display, Formatter};

use chrono::NaiveDate;
use url::Url;

use crate::software_release::SoftwareRelease;

/// An article from the Apple Developer software releases site.
pub struct Article {
    /// The article title. For an OS release, this is the OS version.
    /// Example: iOS 16.2 beta 3 (20C5049e)
    pub title: String,

    /// The article date. Example: November 15, 2022
    pub date: NaiveDate,

    /// The release notes URL. This will be `None` if the article does not have release notes.
    /// Release note URLs end in a path like `/go/?id=ios-16.2-rn`
    pub release_notes_url: Option<Url>,
}

impl Article {
    /// "Unfurls" a releases notes URL to the final page URL. These aren't HTTP redirects,
    /// but rather in-page JavaScript redirects. The final URL path looks something like the
    /// following: `/documentation/ios-ipados-release-notes/ios-ipados-16_2-release-notes`.
    pub fn release_notes_url_unfurled(&self) -> Option<Url> {
        self.release_notes_url
            .as_ref()
            .map(|url| crate::url::unfurl(url).unwrap())
    }

    /// Attempts to parse `title` as an software release with version.
    pub fn software_release() -> Option<SoftwareRelease> {
        None
    }
}

impl Display for Article {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} - {}",
            self.date.format("%Y-%m-%d").to_string(),
            self.title
        )
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[test]
fn test_article_display() {
    let date = NaiveDate::parse_from_str("November 15, 2022", "%B %d, %Y").unwrap();
    let article = Article {
        title: "iOS 16.2 beta 3 (20C5049e)".to_string(),
        date,
        release_notes_url: Url::parse("https://developer.apple.com/go/?id=ios-16.2-rn").ok(),
    };

    assert_eq!(
        article.to_string(),
        "2022-11-15 - iOS 16.2 beta 3 (20C5049e)"
    );
}

#[test]
fn test_article_display_without_url() {
    let date = NaiveDate::parse_from_str("November 15, 2022", "%B %d, %Y").unwrap();
    let article = Article {
        title: "App Store Connect 1.11".to_string(),
        date,
        release_notes_url: None,
    };

    assert_eq!(article.to_string(), "2022-11-15 - App Store Connect 1.11");
}
