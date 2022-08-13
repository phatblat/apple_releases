//
// main.rs
//

/// Executable entry point.
fn main() {
    let apple_dev_news_updates = "https://developer.apple.com/news/releases/";
    let body = get(apple_dev_news_updates.to_string()).unwrap();

    find(body);
}

/// Get a URL and return the body of the response.
fn get(url: String) -> Result<String, Box<dyn std::error::Error>> {
    let res = reqwest::blocking::get(url)?;
    // println!("{:#?}", res);

    let body = res.text()?;
    // println!("{}", body);

    Ok(body)
}

fn find(text: String) {
    use scraper::{Html, Selector};

    let document = Html::parse_document(&text);
    // println!("{:#?}", document);

    let selector_main = Selector::parse(r#"[class*="article-date"]"#).unwrap();
    let main = document.select(&selector_main).next().unwrap();
    println!("{:#?}", main);

    let selector_grid = Selector::parse(r#"section"#).unwrap();
    let _grid = document.select(&selector_grid).next().unwrap();
    // println!("{:#?}", grid);

    let selector = Selector::parse(r#"section.grid"#).unwrap();

    let wrapper = document.select(&selector).next().unwrap();
    println!("matched");
    println!("{:#?}", wrapper);
}
