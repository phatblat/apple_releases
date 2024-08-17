//!
//! url.rs
//!

use url::Url;

use crate::{GenericResult, APPLE_DEV_RELEASES, APP_USER_AGENT};

/// Gets a URL and returns the body of the response.
///
/// # Arguments
///
/// - `url` - The URL to get.
pub fn get(url: String) -> GenericResult<String> {
    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;
    let res = client.get(url).send()?;
    let body = res.text()?;
    Ok(body)
}

/// Builds the release notes URL.
///
/// # Arguments
///
/// - `notes` - The release notes path.
pub(crate) fn build_notes_url(notes_path: Option<String>) -> Option<Url> {
    notes_path.map(|path| {
        let base_url = Url::parse(APPLE_DEV_RELEASES).unwrap();
        base_url.join(&path).unwrap()
    })
}

/* ---------------------------------------------------------------------------------------------- */

#[test]
fn test_get() {
    let body = get(APPLE_DEV_RELEASES.to_string()).unwrap();
    assert!(!body.is_empty());
}

#[test]
fn test_build_notes_url() {
    let expected_url = Url::parse("https://developer.apple.com/go/?id=xcode-14-sdk-rn").unwrap();
    let path = Some("/go/?id=xcode-14-sdk-rn".to_string());
    let url = build_notes_url(path).unwrap();
    assert_eq!(url, expected_url);
}

#[test]
fn test_build_notes_url_with_none() {
    let expected_url = None;
    let url = build_notes_url(None);
    assert_eq!(url, expected_url);
}
