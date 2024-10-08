//!
//! article.rs
//!

use crate::software_release::SoftwareRelease;
use chrono::NaiveDate;
use semver::{BuildMetadata, Prerelease};
use std::fmt::{Display, Formatter};
use url::Url;

/// An article from the Apple Developer software releases site.
pub struct Article {
    /// The article title. For an OS release, this is the OS version.
    /// Example: iOS 16.2 beta 3 (20C5049e)
    pub title: String,

    /// Struct representing the product and version.
    pub software_release: Option<SoftwareRelease>,

    /// The article date. Example: November 15, 2022
    pub date: NaiveDate,

    /// The release notes URL. This will be `None` if the article does not have release notes.
    /// Release note URLs end in a path like `/go/?id=ios-16.2-rn`
    pub release_notes_url: Option<Url>,
}

impl Display for Article {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{} - ", self.date.format("%Y-%m-%d"))?;

        if let Some(SoftwareRelease { product, version }) = &self.software_release {
            write!(formatter, "{} ", product)?;

            // Write semver without trailing zeros
            write!(formatter, "{}", version.major.to_string())?;
            // Include minor if we have a patch version
            if version.minor > 0 || version.patch > 0 {
                write!(formatter, ".{}", version.minor.to_string())?;
            }
            if version.patch > 0 {
                write!(formatter, ".{}", version.patch.to_string())?;
            }

            if version.pre != Prerelease::EMPTY {
                write!(formatter, " {}", version.pre.to_string().replace("-", " "))?;
            }
            if version.build != BuildMetadata::EMPTY {
                write!(formatter, " ({})", version.build.to_string())?;
            }

            if let Some(url) = &self.release_notes_url {
                write!(formatter, " - {}", url.to_string())?;
            }

            // Suppresses error: Type mismatch [E0308] expected `core::fmt::Result`, but found `()`
            write!(formatter, "")
        } else {
            write!(formatter, "{} - {}", self.date.format("%Y-%m-%d"), self.title)
        }
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[test]
fn test_article_display() {
    let date = NaiveDate::parse_from_str("November 15, 2022", "%B %d, %Y").unwrap();

    let article = Article {
        title: "iOS 16.2 beta 3 (20C5049e)".to_string(),
        software_release: None,
        date,
        release_notes_url: Url::parse("https://developer.apple.com/go/?id=ios-16.2-rn").ok(),
    };

    assert_eq!(
        article.to_string(),
        "2022-11-15 - iOS 16.2 beta 3 (20C5049e)"
    );
}

#[test]
fn test_article_display_software_release() {
    let title = "iOS 16.2 beta 3 (20C5049e)";
    let date = NaiveDate::parse_from_str("November 15, 2022", "%B %d, %Y").unwrap();
    let release = SoftwareRelease::software_release(title.to_string()).unwrap();

    let article = Article {
        title: title.to_string(),
        software_release: Some(release),
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
        software_release: None,
        date,
        release_notes_url: None,
    };

    assert_eq!(article.to_string(), "2022-11-15 - App Store Connect 1.11");
}
