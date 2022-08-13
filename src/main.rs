//
// main.rs
//

/// Executable entry point.
fn main() {
    let apple_dev_news_updates = "https://developer.apple.com/news/releases/";
    let body = get(apple_dev_news_updates.to_string())
        .unwrap();
}

/// Get a URL and return the body of the response.
fn get(url: String) -> Result<String, Box<dyn std::error::Error>> {
    let res = reqwest::blocking::get(url)?;
    // println!("{:#?}", res);

    let body = res.text()?;
    println!("{}", body);

    Ok(body)
}
