use scraper::Html;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Serialize, Deserialize)]
pub struct Link(pub String);

impl Deref for Link {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwirLinkElement {
    pub link: Link,
    pub title: String,
}

impl TwirLinkElement {
    pub fn new(link: Link, title: String) -> Self {
        TwirLinkElement { link, title }
    }
}
