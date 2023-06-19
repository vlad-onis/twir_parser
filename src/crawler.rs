use scraper::{Html, Selector};
use thiserror::Error;
use tracing::error;

pub const ISSUE_BACK_BONE_THIS: &str = "this-week-in-rust-";
pub const ISSUE_BACK_BONE_LAST: &str = "last-week-in-rust-";
pub const ISSUE_BACK_BONE_THESE: &str = "these-weeks-in-rust-";

#[derive(Debug, Error)]
pub enum CrawlerError {
    #[error("Request failed because: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Failed to parse a html selector")]
    SelectorParsing,
}

/// This function makes a few assumptions about the underlying Html:
/// 1. Each link starts with  <a href=\" (including the quotes)
/// 2. Each link is valid -> this will be fixed with lychee
/// 3. Each link ends with the quotes and tag "> =>
///
/// Valid link example: <a href="https://this-week-in-rust.org/blog/2020/07/14/this-week-in-rust-347/">This Week in Rust 347</a>
/// Currated link collected by this function: https://this-week-in-rust.org/blog/2019/02/26/this-week-in-rust-275/
pub async fn get_latest_issue_index() -> Result<Vec<String>, CrawlerError> {
    // todo:
    // 1. verify your links with lychee: https://github.com/lycheeverse/lychee
    // 2. Do not return Vec<String> but a Link type that is verified when constructed
    // 4. Logging.
    //

    let client = reqwest::Client::new();
    let origin_url = "https://this-week-in-rust.org/blog/archives/index.html";
    let response = client.get(origin_url).send().await?.text().await?;

    let document = Html::parse_document(&response);

    // Todo: Do not swallow the error here
    let selector = Selector::parse("a").map_err(|_| CrawlerError::SelectorParsing)?;

    let links: Vec<String> = document
        .select(&selector)
        .map(|element| element.html())
        .filter(|element| {
            element.contains(ISSUE_BACK_BONE_THIS)
                || element.contains(ISSUE_BACK_BONE_LAST)
                || element.contains(ISSUE_BACK_BONE_THESE)
        })
        .map(|element| {
            // trim beginnig tag
            let currated_link = element.replace("<a href=\"", "");

            // Abomination that removes all the html after the end of the link
            // It assumes that the html link ends with "> that is why it splits for the quote
            let currated_link = currated_link
                .split('"')
                .collect::<Vec<&str>>()
                .first()
                .unwrap_or(&"")
                .to_owned()
                .to_owned();
            currated_link
        })
        .collect();

    Ok(links)
}

#[allow(dead_code)]
pub async fn inspect() {
    let client = reqwest::Client::new();
    let origin_url = "https://this-week-in-rust.org/blog/archives/index.html";
    let response = client
        .get(origin_url)
        .send()
        .await
        .expect("Failed to get response") // Resolve expect
        .text()
        .await
        .expect("Failed to get payload"); // // Resolve expect

    let document = Html::parse_document(&response);
    println!("{}", document.html());
}
