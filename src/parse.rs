use scraper::{ElementRef, Html, Selector};
use crate::article::Article;
use crate::{GenericResult, SELECTORS};

/// Finds articles in the HTML.
///
/// # Arguments
///
/// - `content` - The HTML to parse.
///
/// # Returns
///
/// A list of articles.
pub fn parse_articles(content: String) -> GenericResult<Vec<Article>> {
    let document = Html::parse_document(&content);
    let mut articles: Vec<Article> = Vec::new();

    for container in document.select(&SELECTORS.article) {
        let title = parse_article_title(&container, &SELECTORS.title).expect("title");
        let date = parse_article_date(&container, &SELECTORS.date).expect("date");
        let notes = parse_release_notes_link(&container, &SELECTORS.release_notes_short_url);
        let notes_url = crate::url::build_notes_url(notes)
            // Ignore Transporter app store links
            .filter(|url| url.as_str().contains("developer.apple.com"))
            .map(|url| crate::url::unfurl(url).unwrap());

        // TODO: Log debug
        // let url = notes_url.as_ref().map_or(None, |url| Some(url.to_string()));
        // println!("{} - {}, <{}>", date, title, url.unwrap_or_default());

        articles.push(Article {
            title,
            date,
            release_notes_url: notes_url,
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
pub fn parse_article_title(element: &ElementRef, selector: &Selector) -> GenericResult<String> {
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
pub fn parse_article_date(element: &ElementRef, selector: &Selector) -> GenericResult<String> {
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
pub fn parse_release_notes_link(element: &ElementRef, selector: &Selector) -> Option<String> {
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
