//
// main.rs
//

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

    for container in document.select(&selector_article_container) {
        // let title = container.select(&selector_article_title).next();
        // println!("{}", title.inner_html());

        match container.select(&selector_article_title).next() {
            Some(title) => println!("{}", title.inner_html()),
            None => continue,
        }

        match container.select(&selector_date).next() {
            Some(date) => println!("{}", date.inner_html()),
            None => continue,
        }
        // println!("{:?}", title.inner_html());

        // let date = container.select(&selector_date).next().unwrap();
        // println!("{:#?}", date.inner_html());
    }
}
