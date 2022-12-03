//!
//! software_release.rs
//!

use semver::{BuildMetadata, Prerelease, Version};

pub struct SoftwareRelease {
    /// Article title.
    pub name: String,

    /// Article date.
    pub version: SoftwareVersion,
}

/// A semantic version number.
pub struct SoftwareVersion {
    /// The major version number.
    pub major: u8,

    /// The minor version number.
    pub minor: u8,

    /// The patch version number.
    pub patch: u8,

    /// The Apple build number which includes letters.
    pub build: String,
}

/*
impl SoftwareVersion {
    /// Parses a version string into a `SoftwareVersion`.
    ///
    /// # Arguments
    ///
    /// - `version` - The version string to parse.
    ///
    /// # Returns
    ///
    /// A `SoftwareVersion` if the string is valid.
    pub fn parse(version: &str) -> Option<SoftwareVersion> {
        let mut parts = version.split('.');

        let major = parts.next()?.parse::<u8>().ok()?;
        let minor = parts.next()?.parse::<u8>().ok()?;
        let patch = parts.next()?.parse::<u8>().ok()?;
        let build = parts.next()?.to_string();

        Some(SoftwareVersion {
            major,
            minor,
            patch,
            build,
        })
    }
}
*/

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
