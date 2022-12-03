//!
//! software_release.rs
//!

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
