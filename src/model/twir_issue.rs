use serde::{Deserialize, Serialize};
use std::{fmt::Display, ops::Deref};
use tabled::Tabled;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Link(pub String);

impl Deref for Link {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Tabled)]
pub struct TwirLinkElement {
    pub title: String,
    pub link: Link,
}

impl TwirLinkElement {
    pub fn new(link: Link, title: String) -> Self {
        TwirLinkElement { link, title }
    }
}

impl Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
