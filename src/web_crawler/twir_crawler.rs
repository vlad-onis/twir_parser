use scraper::{Html, Selector};
use thiserror::Error;
use tracing::{error, info, warn};

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

pub struct TwirCrawler {}

impl TwirCrawler {
    pub fn new() -> Self {
        TwirCrawler {}
    }

    // todo: Return type could be a nicer structure instead of a tuple of string, string
    // The first returned String in the tuple is the link, the other is the title
    //
    // Structure of a link:
    // "<a href=\"https://this-week-in-rust.org/blog/2013/08/10/this-week-in-rust-10/\">This Week in Rust 10</a>"
    fn extract_link_and_title(&self, link: String) -> (String, String) {
        // This function removes the html tags and quotes in the link
        // splitting it into the link part and the title part
        let link = link.replace("<a href=\"", "");

        // Abomination that removes all the html after the end of the link
        // It assumes that the html link ends with "> that is why it splits for the quote
        let elements: Vec<&str> = link.split('"').collect();
        let currated_link = elements.first().unwrap_or(&"").to_string();

        // Abomination that removes all the html bits in the title part
        // It assumes that the title still has ">" and "</a>"
        // It uses replace because in case the characters are not found the title is unaltered
        let title = elements
            .last()
            .unwrap_or(&"")
            .replace("</a>", "")
            .replace(">", "");

        (currated_link, title)
    }

    /// This function makes a few assumptions about the underlying Html:
    /// 1. Each link starts with  <a href=\" (including the quotes)
    /// 2. Each link is valid -> this will be fixed with lychee
    /// 3. Each link ends with the quotes and tag "> =>
    ///
    /// Valid link example: <a href="https://this-week-in-rust.org/blog/2020/07/14/this-week-in-rust-347/">This Week in Rust 347</a>
    /// Currated link collected by this function: https://this-week-in-rust.org/blog/2019/02/26/this-week-in-rust-275/
    pub async fn get_issues_and_titles(&self) -> Result<Vec<(String, String)>, CrawlerError> {
        // todo:
        // 1. verify your links with lychee: https://github.com/lycheeverse/lychee
        // 2. Do not return Vec<String> but a Link type that is verified when constructed

        let client = reqwest::Client::new();
        let origin_url = "https://this-week-in-rust.org/blog/archives/index.html";
        let response = client.get(origin_url).send().await?.text().await?;

        let document = Html::parse_document(&response);

        // Todo: Do not swallow the error here
        let selector = Selector::parse("a").map_err(|_| CrawlerError::SelectorParsing)?;

        let links_and_titles: Vec<(String, String)> = document
            .select(&selector)
            .map(|element| element.html())
            .filter(|element| {
                element.contains(ISSUE_BACK_BONE_THIS)
                    || element.contains(ISSUE_BACK_BONE_LAST)
                    || element.contains(ISSUE_BACK_BONE_THESE)
            })
            // .for_each(|element| info!("{element:?}"))
            .map(|element| self.extract_link_and_title(element))
            .collect();

        Ok(links_and_titles)
    }

    pub async fn search(&self, sentence: String) {
        let issues_and_titles = self.get_issues_and_titles().await.unwrap();

        // link and title
        let mut found_resources: Vec<(String, String)> = Vec::new();

        for (link, _) in issues_and_titles {
            found_resources.append(&mut self.parse_page(&link, &sentence).await)
        }

        error!("{found_resources:?}");
    }

    pub async fn parse_page(&self, origin_url: &str, sentence: &str) -> Vec<(String, String)> {
        let client = reqwest::Client::new();
        let response = client
            .get(origin_url)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let document = Html::parse_document(&response);

        // Todo: Do not swallow the error here
        let selector = Selector::parse("a")
            .map_err(|_| CrawlerError::SelectorParsing)
            .unwrap();

        let links: Vec<(String, String)> = document
            .select(&selector)
            .map(|element| element.html())
            .map(|element| self.extract_link_and_title(element))
            .filter(|(_, title)| title.contains(sentence)) // just an example from 440th issue
            .collect();

        links
    }

    #[allow(dead_code)]
    async fn inspect() {
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
}
