//!
//! article.rs
//!

use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use chrono::NaiveDate;
use semver::{BuildMetadata, Prerelease};
use url::Url;

use crate::product::Product;
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
    pub(crate) fn release_notes_url_unfurled(&self) -> Option<Url> {
        self.release_notes_url
            .as_ref()
            .map(|url| crate::url::unfurl(url).unwrap())
    }

    /// Attempts to parse `title` as a software release with version.
    #[allow(dead_code)]
    pub(crate) fn software_release(&self) -> Option<SoftwareRelease> {
        // iOS 16.2 beta 4 (20C5058d)
        // iOS 16.1.2 (20B110)

        let mut tokens = self.title.split(" ");
        // for token in tokens {}
        let product_name = tokens.next().unwrap();

        Product::from_str(product_name)
            .map(|product| {
                let mut version_string = tokens.next().unwrap().to_string();
                let mut tmp_string: &str = tokens.next().unwrap();
                while !tmp_string.contains("(") {
                    version_string = format!("{}-{}", version_string, tmp_string);
                    tmp_string = tokens.next().unwrap();
                }

                if tmp_string.contains("(") {
                    // Crop parentheses off both ends
                    let build_string = &tmp_string[1..tmp_string.len() - 1];
                    version_string = format!("{}+{}", version_string, build_string);
                }

                let version = lenient_semver::parse(&*version_string).unwrap();
                Some(SoftwareRelease { product, version })
            })
            .unwrap_or(None)
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

#[test]
fn test_software_prerelease() {
    let date = NaiveDate::parse_from_str("November 15, 2022", "%B %d, %Y").unwrap();
    let article = Article {
        title: "iOS 16.2 beta 3 (20C5049e)".to_string(),
        date,
        release_notes_url: None,
    };

    let release = article.software_release().unwrap();
    assert_eq!(release.product, Product::iOS);
    assert_eq!(release.version.major, 16);
    assert_eq!(release.version.minor, 2);
    assert_eq!(release.version.patch, 0);
    assert_eq!(release.version.pre, Prerelease::new("beta-3").unwrap());
    assert_eq!(
        release.version.build,
        BuildMetadata::new("20C5049e").unwrap()
    );
    assert_eq!(release.version.to_string(), "16.2.0-beta-3+20C5049e");
}

#[test]
fn test_software_release() {
    let date = NaiveDate::parse_from_str("November 15, 2022", "%B %d, %Y").unwrap();
    let article = Article {
        title: "iOS 16.1.2 (20B110)".to_string(),
        date,
        release_notes_url: None,
    };

    let release = article.software_release().unwrap();
    assert_eq!(release.product, Product::iOS);
    assert_eq!(release.version.major, 16);
    assert_eq!(release.version.minor, 1);
    assert_eq!(release.version.patch, 2);
    assert_eq!(release.version.pre, Prerelease::new("").unwrap());
    assert_eq!(release.version.build, BuildMetadata::new("20B110").unwrap());
    assert_eq!(release.version.to_string(), "16.1.2+20B110");
}
