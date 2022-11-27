//!
//! selectors.rs
//!

use scraper::Selector;

/// Collection of scraper Selectors - https://docs.rs/scraper/latest/scraper/
pub(crate) struct Selectors {
    /// Parses the article container, the top-level containing values of interest.
    pub(crate) article: Selector,
    /// Parses the article title.
    pub(crate) title: Selector,
    /// Parses the article date.
    pub(crate) date: Selector,
    /// Parses the short release notes URL.
    pub(crate) release_notes_short_url: Selector,
    /// Parses the release notes URL inside a script tag.
    pub(crate) release_notes_full_url: Selector,
}

impl Selectors {
    pub(crate) fn new() -> Self {
        Self {
            article: Selector::parse(r#"section.article-content-container"#).unwrap(),
            title: Selector::parse(r#"a.article-title h2"#).unwrap(),
            date: Selector::parse(r#"p.article-date"#).unwrap(),
            release_notes_short_url: Selector::parse(r#"span.article-text il a.more"#).unwrap(),
            // Selects the last of 4 sibling script tags.
            release_notes_full_url: Selector::parse(r#"script + script + script + script"#).unwrap(),
        }
    }
}
