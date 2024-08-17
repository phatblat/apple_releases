//!
//! software_release.rs
//!

use crate::product::Product;
use semver::Version;
use std::str::FromStr;

pub(crate) struct SoftwareRelease {
    /// Known product type.
    pub(crate) product: Product,

    /// Product version.
    pub(crate) version: Version,
}

impl SoftwareRelease {
    /// Attempts to parse `title` as a software release with version.
    pub(crate) fn software_release(title: String) -> Option<SoftwareRelease> {
        // iOS 16.2 beta 4 (20C5058d)
        // iOS 16.1.2 (20B110)

        let mut tokens = title.split(' ');
        // for token in tokens {}
        let product_name = tokens.next().unwrap();

        Product::from_str(product_name)
            .map(|product| {
                let mut version_string = tokens.next().unwrap().to_string();
                let mut tmp_string: &str = tokens.next().unwrap();
                while !tmp_string.contains('(') {
                    version_string = format!("{}-{}", version_string, tmp_string);
                    tmp_string = tokens.next().unwrap();
                }

                if tmp_string.contains('(') {
                    // Crop parentheses off both ends
                    let build_string = &tmp_string[1..tmp_string.len() - 1];
                    version_string = format!("{}+{}", version_string, build_string);
                }

                let version = lenient_semver::parse(&version_string).unwrap();
                Some(SoftwareRelease { product, version })
            })
            .unwrap_or(None)
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[test]
fn test_semver_prerelease_parsing() {
    // iOS 16.2 beta 4 (20C5058d)
    let version_string = "16.2-beta.4+20C5058d";
    let version = lenient_semver::parse(version_string).unwrap();
    assert_eq!(version.major, 16);
    assert_eq!(version.minor, 2);
    assert_eq!(version.patch, 0);
    assert_eq!(version.pre, semver::Prerelease::new("beta.4").unwrap());
    assert_eq!(
        version.build,
        semver::BuildMetadata::new("20C5058d").unwrap()
    );
}

#[test]
fn test_software_prerelease() {
    let title = "iOS 16.2 beta 3 (20C5049e)";
    let release = SoftwareRelease::software_release(title.into()).unwrap();

    assert_eq!(release.product, Product::iOS);
    assert_eq!(release.version.major, 16);
    assert_eq!(release.version.minor, 2);
    assert_eq!(release.version.patch, 0);
    assert_eq!(
        release.version.pre,
        semver::Prerelease::new("beta-3").unwrap()
    );
    assert_eq!(
        release.version.build,
        semver::BuildMetadata::new("20C5049e").unwrap()
    );
    assert_eq!(release.version.to_string(), "16.2.0-beta-3+20C5049e");
}

#[test]
fn test_software_release() {
    let title = "iOS 16.1.2 (20B110)";
    let release = SoftwareRelease::software_release(title.into()).unwrap();

    assert_eq!(release.product, Product::iOS);
    assert_eq!(release.version.major, 16);
    assert_eq!(release.version.minor, 1);
    assert_eq!(release.version.patch, 2);
    assert_eq!(release.version.pre, semver::Prerelease::new("").unwrap());
    assert_eq!(
        release.version.build,
        semver::BuildMetadata::new("20B110").unwrap()
    );
    assert_eq!(release.version.to_string(), "16.1.2+20B110");
}
