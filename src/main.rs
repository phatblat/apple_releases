//
// main.rs
//

static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

/// Executable entry point.
fn main() {
    let apple_dev_news_updates = "https://developer.apple.com/news/releases/";
    let body = get(apple_dev_news_updates.to_string()).unwrap();

    find(body);
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

fn find(text: String) {
    use scraper::{Html, Selector};

    let document = Html::parse_document(&text);
    // println!("{:#?}", document);

    let selector_article_title = Selector::parse(r#"a[class="article-title external-link"] h2"#).unwrap();
    for title in document.select(&selector_article_title) {
        println!("{}", title.inner_html());
    }

    let selector_date = Selector::parse(r#"[class*="article-date"]"#).unwrap();
    let date = document.select(&selector_date).next().unwrap();
    // println!("{:#?}", date);


}
