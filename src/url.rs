//!
//! url.rs
//!

use scraper::Html;
use url::Url;
use crate::{APP_USER_AGENT, APPLE_DEV_RELEASES, GenericError, GenericResult, SELECTORS};
use crate::parse::parse_article_title;

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
    // println!("{:#?}", res);

    let body = res.text()?;
    // println!("{}", body);

    Ok(body)
}

/// Unfurls a release notes URL. These URLs use in-page JavaScript to redirect to the actual URL.
///
/// ```
/// <script>
/// window.setTimeout("window.location.replace('/documentation/ios-ipados-release-notes/ios-ipados-16_2-release-notes')", 1);
/// </script>
/// ```
///
/// # Arguments
///
/// - `url` - The URL at the end of a redirect chain.
pub(crate) fn unfurl(url: Url) -> GenericResult<Url> {
    let body = crate::url::get(url.to_string()).unwrap();
    let document = Html::parse_document(&body);

    let script = parse_article_title(&document.root_element(), &SELECTORS.release_notes_full_url)
        .unwrap();

    let tokens = script.split("'");
    let path = tokens.take(2).last().unwrap();

    // TODO: Log debug
    // println!("{:?}", path);

    let releases_url = Url::parse(APPLE_DEV_RELEASES).unwrap();
    let base = base_url(releases_url).unwrap();
    let full_url = base.join(path).unwrap();

    // TODO: Log debug
    // println!("{}", full_url);

    Ok(full_url)
}

/// Builds the release base URL.
///
/// # Arguments
///
/// - `url` - The release notes path.
fn base_url(mut url: Url) -> GenericResult<Url> {
    match url.path_segments_mut() {
        Ok(mut path) => {
            path.clear();
        }
        Err(_) => {
            return Err(GenericError::try_from("Error extracting base URL").unwrap());
        }
    }

    url.set_query(None);

    Ok(url)
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