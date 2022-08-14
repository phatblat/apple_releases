//!
//! main.rs
//!

use scraper::{Html, Selector};
use scraper::html::Select;

static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

/// Executable entry point.
fn main() {
    let apple_dev_news_updates = "https://developer.apple.com/news/releases/";
    let body = get(apple_dev_news_updates.to_string()).unwrap();

    find_articles(body);
}

/// Get a URL and return the body of the response.
fn get(url: String) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;
    let res = client.get(url).send()?;
    // println!("{:#?}", res);

    let body = res.text()?;
    // println!("{}", body);

    Ok(body)
}

fn find_articles(content: String) {
    let selector_article_container = Selector::parse(r#"section[class~="article-content-container"]"#).unwrap();

    let document = Html::parse_document(&content);
    // println!("{:#?}", document);

    // let container = document.select( & selector_article_container);

    let selector_article_title = Selector::parse(r#"a[class="article-title external-link"] h2"#).unwrap();
    let selector_date = Selector::parse(r#"p[class*="article-date"]"#).unwrap();
    let selector_notes = Selector::parse(r#"span[class*="article-text"] il a.more"#).unwrap();

    for container in document.select(&selector_article_container) {
        match container.select(&selector_article_title).next() {
            Some(title) => println!("{}", title.inner_html()),
            None => continue,
        }

        match container.select(&selector_date).next() {
            Some(date) => println!("{}", date.inner_html()),
            None => continue,
        }

        match container.select(&selector_notes).next() {
            Some(notes) => match notes.value().attr("href") {
                Some(href) => println!("{}", href),
                None => continue,
            },
            None => continue,
        }
    }
}

#[test]
fn test_get() {
    let apple_dev_news_updates = "https://developer.apple.com/news/releases/";
    let body = get(apple_dev_news_updates.to_string()).unwrap();
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

    find_articles(html);
}
