//!
//! article.rs
//!

use std::fmt::{Display, Formatter};
use url::Url;

/// An article from the Apple Developer software releases site.
pub struct Article {
    pub title: String,
    pub date: String,
    pub release_notes_url: Option<Url>,
}

impl Display for Article {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let url = self.release_notes_url.as_ref().map_or(None, |url| Some(url.to_string()));
        write!(f, "{} - {}, <{}>", self.date, self.title, url.unwrap_or_default())
    }
}
