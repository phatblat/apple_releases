//!
//! url.rs
//!

use scraper::Html;
use url::Url;

use crate::parse::parse_article_title;
use crate::{GenericError, GenericResult, APPLE_DEV_RELEASES, APP_USER_AGENT, SELECTORS};

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
pub(crate) fn unfurl(url: &Url) -> GenericResult<Url> {
    let body = get(url.to_string()).unwrap();
    let document = Html::parse_document(&body);

    let script =
        parse_article_title(&document.root_element(), &SELECTORS.release_notes_full_url).unwrap();

    let tokens = script.split('\'');
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

#[test]
fn test_parse_redirect_script() {
    let html = r###"
    <!-- metrics -->
    <script>
        /* RSID: */
        var s_account="awdappledeveloper"
    </script>
    <script src="/assets/metrics/scripts/analytics.js?10202020"></script>
    <script>
        s.pageName= AC && AC.Tracking && AC.Tracking.pageName();
        s.channel="www.en.developer"
        s.channel="www.en.developer";


        /************* DO NOT ALTER ANYTHING BELOW THIS LINE ! **************/
        var s_code=s.t();if(s_code)document.write(s_code)
    </script>
    <!-- /metrics -->
    <script>
    window.setTimeout("window.location.replace('/documentation/ios-ipados-release-notes/ios-ipados-16_2-release-notes')", 1);
    </script>
    "###.to_string();

    let fragment = Html::parse_fragment(&html);

    // test parsing using local selector
    let selector = scraper::Selector::parse(r#"script + script + script + script"#).unwrap();
    let element = fragment.select(&selector).next().unwrap();
    println!("{}", element.inner_html());

    let script =
        parse_article_title(&fragment.root_element(), &SELECTORS.release_notes_full_url).unwrap();

    assert_eq!(
        script.trim(),
        r#"window.setTimeout("window.location.replace('/documentation/ios-ipados-release-notes/ios-ipados-16_2-release-notes')", 1);"#
    );
}
