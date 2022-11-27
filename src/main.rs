//!
//! main.rs
//!

use std::string::ToString;
use lazy_static::lazy_static;

use crate::selectors::Selectors;

mod article;
mod selectors;
mod url;
mod parse;

/* ---------------------------------------------------------------------------------------------- */

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

/// User agent for network requests.
const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// Webpage listing new software releases.
const APPLE_DEV_RELEASES: &str = "https://developer.apple.com/news/releases/";

lazy_static! {
    static ref SELECTORS: Selectors = Selectors::new();
}

/* ---------------------------------------------------------------------------------------------- */

/// Executable entry point.
fn main() {
    let body = url::get(APPLE_DEV_RELEASES.to_string()).unwrap();

    let articles = parse::parse_articles(body).unwrap();

    articles.iter().for_each(|article| {
        println!("{}", article);
    });
}

/* ---------------------------------------------------------------------------------------------- */

/* ---------------------------------------------------------------------------------------------- */

#[test]
fn test_get() {
    let body = url::get(APPLE_DEV_RELEASES.to_string()).unwrap();
    assert!(body.len() > 0);
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
    let selector = Selector::parse(r#"script + script + script + script"#).unwrap();
    let element = fragment.select(&selector).next().unwrap();
    println!("{}", element.inner_html());

    let script = parse::parse_article_title(&fragment.root_element(), &SELECTORS.release_notes_full_url)
        .unwrap();

    assert_eq!(
        script.trim(),
        r#"window.setTimeout("window.location.replace('/documentation/ios-ipados-release-notes/ios-ipados-16_2-release-notes')", 1);"#
    );
}

#[test]
fn test_unfurl() {
    let expected_url = Url::parse("https://developer.apple.com/documentation/xcode-release-notes/xcode-14-release-notes").unwrap();
    let url = Url::parse("https://developer.apple.com/go/?id=xcode-14-sdk-rn").unwrap();
    let redirect_url = unfurl(url).unwrap();
    assert_eq!(redirect_url, expected_url);
}

#[test]
fn test_parse() {
    let html = r###"
<section class="article-content-container column large-9 medium-9 small-12 no-padding-left padding-right-small padding-top padding-bottom-small divider-top">
    <a class="article-title external-link" href="/download/"><h2>Xcode 14 beta 5 (14A5294e)</h2></a>
    <div class="article-text-wrapper">
        <p class="lighter  article-date">August 8, 2022</p>
        <span class="article-text">
            <p> <a href="/download/applications" class="more">View downloads</a></p>
            <il>
                <p><a href="/go/?id=xcode-14-sdk-rn" class="more">View release notes</a></p>
            </il>
        </span>
    </div>
    <section class="social-share-container">
        <ul class="sharesheet-options">
            <li class="social-option">
                <button class="icon icon-facebook social-icon" data-href="https://developer.apple.com/news/releases/?id=08082022a" data-share-type="facebook" aria-label="Share via Facebook"> </button>
            </li>
            <li class="social-option">
                <button class="icon icon-twitter social-icon" data-href="https://developer.apple.com/news/releases/?id=08082022a" data-share-type="twitter" aria-label="Share via Twitter"></button>
            </li>
            <li class="social-option">
                <button class="icon icon-mail social-icon" data-href="https://developer.apple.com/news/releases/?id=08082022a" data-share-type="mail" data-title="Xcode 14 beta 5 (14A5294e) - Releases - Apple Developer" data-description="https://developer.apple.com/news/releases/?id=08082022a" aria-label="Share via mail"></button>
            </li>
            <li class="social-option">
                <button class="icon icon-link social-icon" data-href="https://developer.apple.com/news/releases/?id=08082022a" data-share-type="copy" data-copy-title="Copied to clipboard" aria-label="Share via link"></button>
            </li>
        </ul>
    </section>
</section>
    "###.to_string();

    let articles = parse::parse_articles(html).expect("Err collecting articles");
    assert_eq!(articles.len(), 1);
}

#[test]
fn test_parse_title() {
    let html = r###"
    <a class="article-title external-link" href="/download/"><h2>Xcode 14 beta 5 (14A5294e)</h2></a>
    "###.to_string();

    let fragment = Html::parse_fragment(&html);

    // test parsing using local selector
    let selector = Selector::parse(r#"a.article-title h2"#).unwrap();
    let element = fragment.select(&selector).next().unwrap();
    println!("{}", element.inner_html());

    let title = parse::parse_article_title(&fragment.root_element(), &SELECTORS.title).unwrap();

    assert_eq!(title, "Xcode 14 beta 5 (14A5294e)");
}

#[test]
fn test_parse_date() {
    let html = r###"
        <p class="lighter  article-date">August 8, 2022</p>
    "###.to_string();

    let fragment = Html::parse_fragment(&html);

    // test parsing using local selector
    let selector = Selector::parse(r#"p.article-date"#).unwrap();
    let element = fragment.select(&selector).next().unwrap();
    println!("{}", element.inner_html());

    let date = parse::parse_article_date(&fragment.root_element(), &SELECTORS.date).unwrap();

    assert_eq!(date, "August 8, 2022");
}

#[test]
fn test_parse_release_notes_link() {
    let html = r###"
        <span class="article-text">
            <p> <a href="/download/applications" class="more">View downloads</a></p>
            <il>
                <p><a href="/go/?id=xcode-14-sdk-rn" class="more">View release notes</a></p>
            </il>
        </span>
    "###.to_string();

    let fragment = Html::parse_fragment(&html);

    // test parsing using local selector
    let selector = Selector::parse(r#"span.article-text il a.more"#).unwrap();
    let element = fragment.select(&selector).next().unwrap().value();
        // .ok_or("No href attribute found")?
        // .to_string()
    println!("{}", element.attr("href").unwrap());

    let notes_url = parse::parse_release_notes_link(&fragment.root_element(), &SELECTORS.release_notes_short_url).unwrap();

    assert_eq!(notes_url, "/go/?id=xcode-14-sdk-rn");
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
fn test_article_display() {
    let article = Article {
        title: "iOS 16.2 beta 3 (20C5049e)".to_string(),
        date: "November 15, 2022".to_string(),
        release_notes_url: Url::parse("https://developer.apple.com/go/?id=ios-16.2-rn").ok(),
    };

    assert_eq!(
        article.to_string(),
        "November 15, 2022 - iOS 16.2 beta 3 (20C5049e), <https://developer.apple.com/go/?id=ios-16.2-rn>"
    );
}

#[test]
fn test_article_display_without_url() {
    let article = Article {
        title: "App Store Connect 1.11".to_string(),
        date: "November 15, 2022".to_string(),
        release_notes_url: None,
    };

    assert_eq!(
        article.to_string(),
        "November 15, 2022 - App Store Connect 1.11, <>"
    );
}
