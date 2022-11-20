//!
//! main.rs
//!

use std::string::ToString;
use lazy_static::lazy_static;
use scraper::{ElementRef, Html, Selector};

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

/// User agent for network requests.
static APP_USER_AGENT: &str = concat!(
env!("CARGO_PKG_NAME"),
"/",
env!("CARGO_PKG_VERSION"),
);

/// An article from the Apple Developer software releases site.
struct Article {
    title: String,
    date: String,
    release_notes_url: Option<String>,
}

/* ---------------------------------------------------------------------------------------------- */

/// Collection of scraper Selectors - https://docs.rs/scraper/latest/scraper/
struct Selectors {
    /// Parses the article container, the top-level containing values of interest.
    article: Selector,
    /// Parses the article title.
    title: Selector,
    /// Parses the article date.
    date: Selector,
    /// Parses the release notes link.
    release_notes_url: Selector,
}

impl Selectors {
    fn new() -> Self {
        Self {
            article: Selector::parse(r#"section.article-content-container"#).unwrap(),
            title: Selector::parse(r#"a.article-title h2"#).unwrap(),
            date: Selector::parse(r#"p.article-date"#).unwrap(),
            release_notes_url: Selector::parse(r#"span.article-text il a.more"#).unwrap(),
        }
    }
}

lazy_static! {
    static ref SELECTORS: Selectors = Selectors::new();
}

/* ---------------------------------------------------------------------------------------------- */

const APPLE_DEV_RELEASES: &str = "https://developer.apple.com/news/releases/";

/// Executable entry point.
fn main() {
    let body = get(APPLE_DEV_RELEASES.to_string()).unwrap();

    let articles = find_articles(body).unwrap();

    articles.iter().for_each(|article| {
        println!("Title: {}", article.title);
        println!("Date: {}", article.date);
        println!("Release Notes: {:?}", article.release_notes_url);
    });
}

/* ---------------------------------------------------------------------------------------------- */

/// Gets a URL and returns the body of the response.
///
/// # Arguments
///
/// - `url` - The URL to get.
fn get(url: String) -> GenericResult<String> {
    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;
    let res = client.get(url).send()?;
    // println!("{:#?}", res);

    let body = res.text()?;
    // println!("{}", body);

    Ok(body)
}

/// Finds articles in the HTML.
///
/// # Arguments
///
/// - `content` - The HTML to parse.
///
/// # Returns
///
/// A list of articles.
fn find_articles(content: String) -> GenericResult<Vec<Article>> {
    let document = Html::parse_document(&content);
    let mut articles: Vec<Article> = Vec::new();

    for container in document.select(&SELECTORS.article) {
        let title = parse_article_title(&container, &SELECTORS.title).expect("title");
        let date = parse_article_date(&container, &SELECTORS.date).expect("date");
        println!("{} - {}", title, date);
        let notes = parse_release_notes_link(&container, &SELECTORS.release_notes_url);

        articles.push(Article {
            title,
            date,
            release_notes_url: notes,
        });
    }

    Ok(articles)
}

/// Parses the article title.
///
/// # Arguments
///
/// - `element` - The HTML ElementRef to parse.
/// - `selector` - The selector to use.
fn parse_article_title(element: &ElementRef, selector: &Selector) -> GenericResult<String> {
    Ok(
        element
            .select(selector)
            .next()
            .ok_or("No title found")?
            .inner_html(),
    )
}

/// Parses the article date.
///
/// # Arguments
///
/// - `element` - The HTML ElementRef to parse.
/// - `selector` - The selector to use.
fn parse_article_date(element: &ElementRef, selector: &Selector) -> GenericResult<String> {
    Ok(
        element
            .select(selector)
            .next()
            .ok_or("No date found")?
            .inner_html(),
    )
}

/// Parses the release notes link.
///
/// # Arguments
///
/// - `element` - The HTML ElementRef to parse.
/// - `selector` - The selector to use.
fn parse_release_notes_link(element: &ElementRef, selector: &Selector) -> Option<String> {
    match element
            .select(selector)
            .next()
            .ok_or("No release notes link found")
            .ok()?
            .value()
            .attr("href") {
        Some(url) => Some(url.to_string()),
        None => None,
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[test]
fn test_get() {
    let body = get(APPLE_DEV_RELEASES.to_string()).unwrap();
    assert!(body.len() > 0);
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

    let articles = find_articles(html).expect("Err collecting articles");
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

    let title = parse_article_title(&fragment.root_element(), &SELECTORS.title).unwrap();

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

    let date = parse_article_date(&fragment.root_element(), &SELECTORS.date).unwrap();

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

    let notes_url = parse_release_notes_link(&fragment.root_element(), &SELECTORS.release_notes_url).unwrap();

    assert_eq!(notes_url, "/go/?id=xcode-14-sdk-rn");
}
