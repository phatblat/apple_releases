//!
//! software_release.rs
//!

use semver::{Version};

use crate::product::Product;

pub(crate) struct SoftwareRelease {
    /// Known product type.
    pub(crate) product: Product,

    /// Product version.
    pub(crate) version: Version,
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
    assert_eq!(version.pre, Prerelease::new("beta.4").unwrap());
    assert_eq!(version.build, BuildMetadata::new("20C5058d").unwrap());
}
