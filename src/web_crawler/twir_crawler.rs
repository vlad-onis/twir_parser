use futures::stream::FuturesUnordered;
use futures::StreamExt;
use scraper::{Html, Selector};
use std::sync::Arc;
use std::{fs::File, io::BufWriter, path::Path};
use thiserror::Error;
use tokio::sync::Semaphore;
use tracing::{error, info, trace};

use crate::model::twir_issue::{Link, TwirLinkElement};
use crate::utils::get_progress_bar;

pub const ISSUE_BACK_BONE_THIS: &str = "this-week-in-rust-";
pub const ISSUE_BACK_BONE_LAST: &str = "last-week-in-rust-";
pub const ISSUE_BACK_BONE_THESE: &str = "these-weeks-in-rust-";
pub const TWIR_CONTENTS_FILE_PATH: &str = "twir_content.json";
pub const UNLIMITED: i32 = i32::MAX;

#[derive(Debug, Error)]
pub enum CrawlerError {
    #[error("Request failed because: {0}")]
    Request(#[from] reqwest::Error),

    #[error("Failed to parse a html selector")]
    SelectorParsing,

    #[error("Fetching twir content failed")]
    Fetch,

    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Serde Json error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("Async error: {0}")]
    Async(#[from] tokio::task::JoinError),
}

#[derive(Debug, Default)]
pub struct TwirCrawler {}

impl TwirCrawler {
    // todo: Return type could be a nicer structure instead of a tuple of string, string
    // The first returned String in the tuple is the link, the other is the title
    //
    // Structure of a link:
    // "<a href=\"https://this-week-in-rust.org/blog/2013/08/10/this-week-in-rust-10/\">This Week in Rust 10</a>"
    fn extract_link_and_title(link: String) -> TwirLinkElement {
        // This function removes the html tags and quotes in the link
        // splitting it into the link part and the title part
        let link = link.replace("<a href=\"", "");

        // Abomination that removes all the html after the end of the link
        // It assumes that the html link ends with "> that is why it splits for the quote
        let elements: Vec<&str> = link.split('"').collect();
        let currated_link = elements.first().unwrap_or(&"").to_string();

        let issue_link = Link(currated_link);

        // Abomination that removes all the html bits in the title part
        // It assumes that the title still has ">" and "</a>"
        // It uses replace because in case the characters are not found the title is unaltered
        let title = elements
            .last()
            .unwrap_or(&"")
            .replace("</a>", "")
            .replace('>', "");

        TwirLinkElement::new(issue_link, title)
    }

    /// This function makes a few assumptions about the underlying Html:
    /// -> Fetches all the link elements from: "https://this-week-in-rust.org/blog/archives/index.html"
    /// -> Each link starts with  <a href=\" (including the quotes)
    /// -> Each link is valid -> this will be fixed with lychee
    /// -> Each link ends with the quotes and tag ">
    ///
    /// Valid link example: <a href="https://this-week-in-rust.org/blog/2020/07/14/this-week-in-rust-347/">This Week in Rust 347</a>
    /// Currated link collected by this function: https://this-week-in-rust.org/blog/2019/02/26/this-week-in-rust-275/
    pub async fn get_all_archived_twir_issues(&self) -> Result<Vec<TwirLinkElement>, CrawlerError> {
        // todo:
        // 1. verify your links with lychee: https://github.com/lycheeverse/lychee

        let client = reqwest::Client::new();
        let origin_url = "https://this-week-in-rust.org/blog/archives/index.html";
        let response = client.get(origin_url).send().await?;
        let response_body = response.text().await?;

        let document = Html::parse_document(&response_body);

        // todo: logging the error here is not ideal,
        // we should incorporate the [SelectorErrorKind] in
        // our CrawlerError::SelectorParsing
        let selector = Selector::parse("a").map_err(|e| {
            error!("Selector parsing failed: {e}");
            CrawlerError::SelectorParsing
        })?;

        // Only selects issue links from the archive
        // the [ISSUE_BACK_BONE_THIS], [ISSUE_BACK_BONE_LAST] and [ISSUE_BACK_BONE_THESE]
        // exist because of naming inconsistencies of the issue titles
        let links_and_titles: Vec<TwirLinkElement> = document
            .select(&selector)
            .map(|element| element.html())
            .filter(|element| {
                element.contains(ISSUE_BACK_BONE_THIS)
                    || element.contains(ISSUE_BACK_BONE_LAST)
                    || element.contains(ISSUE_BACK_BONE_THESE)
            })
            .map(Self::extract_link_and_title)
            .collect();

        Ok(links_and_titles)
    }

    async fn lychee_filter_issues(&self, issues: &mut Vec<TwirLinkElement>) {
        let progress_bar = get_progress_bar("Verifying Links");

        let bar_value: f32 = 100.0 / issues.len() as f32;
        let mut current_bar_value = 0.0;

        for (index, resource) in issues.clone().into_iter().enumerate() {
            if lychee_lib::check(resource.link.0)
                .await
                .unwrap()
                .status()
                .is_failure()
            {
                issues.remove(index);
            }
            current_bar_value += bar_value;
            let rounded_bar_value = current_bar_value.round() as u64;
            progress_bar.inc(rounded_bar_value - progress_bar.position());
        }
        progress_bar.finish_with_message("Done");
    }

    pub async fn search_offline(
        &self,
        sentence: &str,
    ) -> Result<Vec<TwirLinkElement>, CrawlerError> {
        let file_contents = std::fs::read_to_string(TWIR_CONTENTS_FILE_PATH)?;
        let issues_and_titles = serde_json::from_str::<Vec<TwirLinkElement>>(&file_contents)?;

        let mut found_resources: Vec<TwirLinkElement> = issues_and_titles
            .into_iter()
            .filter(|issue| issue.title.contains(sentence))
            .collect();

        self.lychee_filter_issues(&mut found_resources).await;

        let len = found_resources.len();

        info!("Issues found offline: {}", len);

        Ok(found_resources)
    }

    pub async fn search_online(
        &self,
        sentence: &str,
        limit: i32,
    ) -> Result<Vec<TwirLinkElement>, CrawlerError> {
        let issues_and_titles: Vec<TwirLinkElement> = self.get_all_archived_twir_issues().await?;
        let progress_bar = get_progress_bar("Checking Issues");
        let limit = limit as usize;
        let bar_value: f32 = 100.0 / limit as f32;

        let semaphore = Arc::new(Semaphore::new(100)); // adjust as needed
        let mut tasks = FuturesUnordered::new();

        for (index, issue) in issues_and_titles.into_iter().enumerate().take(limit) {
            let link = issue.link.clone();
            let sentence = sentence.to_owned();
            let sem_clone = Arc::clone(&semaphore);

            tasks.push(tokio::spawn(async move {
                let _permit = sem_clone
                    .acquire()
                    .await
                    .expect("Failed to acquire semaphore");
                let result = Self::parse_page(&link, &sentence).await;
                drop(_permit); // ensure the semaphore is released

                (index, result)
            }));
        }

        let mut found_resources: Vec<TwirLinkElement> = Vec::new();
        let mut current_bar_value = 0.0;

        while let Some(result) = tasks.next().await {
            match result {
                Ok((index, Ok(mut resources))) => {
                    found_resources.append(&mut resources);

                    current_bar_value += bar_value;

                    let rounded_bar_value = current_bar_value.round() as u64;
                    progress_bar.inc(rounded_bar_value - progress_bar.position());

                    if index >= limit {
                        trace!("Search limit reached.");
                        break;
                    }
                }
                Ok((_, Err(e))) => return Err(e),
                Err(e) => return Err(CrawlerError::from(e)), // adjust according to your error type
            }
        }

        progress_bar.finish_with_message("Done");
        self.lychee_filter_issues(&mut found_resources).await;

        info!("Issues found online: {}", found_resources.len());

        Ok(found_resources)
    }

    pub async fn search(&self, sentence: String, online: bool, limit: i32) {
        let found = if (!std::path::Path::is_file(Path::new(TWIR_CONTENTS_FILE_PATH))) || online {
            self.search_online(&sentence, limit)
                .await
                .unwrap_or_default()
        } else {
            self.search_offline(&sentence).await.unwrap_or_default()
        };

        for element in found {
            info!("Found: {} -> {}", element.title, element.link.0);
        }
    }

    /// This function receives an actual issue page, fetches its contents
    /// Parses it for links only since we are interested to find articles
    ///
    /// It will return to the user a list of curated links and titles
    /// that the user can search through
    pub async fn parse_page(
        origin_url: &str,
        sentence: &str,
    ) -> Result<Vec<TwirLinkElement>, CrawlerError> {
        let client = reqwest::Client::new();
        let response = client.get(origin_url).send().await?.text().await?;

        let document = Html::parse_document(&response);

        // todo: logging the error here is not ideal,
        // we should incorporate the [SelectorErrorKind] in
        // our CrawlerError::SelectorParsing
        let selector = Selector::parse("a").map_err(|e| {
            error!("Selector parsing failed: {e}");
            CrawlerError::SelectorParsing
        })?;

        let links: Vec<TwirLinkElement> = document
            .select(&selector)
            .map(|element| element.html())
            .map(Self::extract_link_and_title)
            .filter(|issue| issue.title.contains(sentence))
            .collect();

        trace!("{} link/s found on page {}", links.len(), origin_url);

        Ok(links)
    }

    pub async fn get_page_content(
        &self,
        origin_url: &str,
    ) -> Result<Vec<TwirLinkElement>, CrawlerError> {
        let client = reqwest::Client::new();
        let response = client.get(origin_url).send().await?.text().await?;

        let document = Html::parse_document(&response);

        let selector = Selector::parse("a").map_err(|e| {
            error!("Selector parsing failed: {e}");
            CrawlerError::SelectorParsing
        })?;

        let links: Vec<TwirLinkElement> = document
            .select(&selector)
            .map(|element| element.html())
            .map(Self::extract_link_and_title)
            .collect();

        Ok(links)
    }

    /// This function fetches the contents from all the archived issues on TWIR
    /// It will serialize them in a file at path [TWIR_CONTENTS_FILE_PATH]
    ///
    /// Note: This function takes a lot of time run as it needs to fetch every single page
    pub async fn fetch_and_save_twir(&self) -> Result<(), CrawlerError> {
        info!("Fetch started");

        let twir_issues = self.get_all_archived_twir_issues().await?;
        let twir_issues_len = twir_issues.len();
        info!("Fetched {} issues.", twir_issues_len);

        let mut full_contents: Vec<TwirLinkElement> = Vec::new();

        for issue in twir_issues {
            full_contents.append(&mut self.get_page_content(&issue.link).await?);
        }

        let file = File::create(TWIR_CONTENTS_FILE_PATH).map_err(|e| {
            error!("Creating the twir content file failed: {e}");
            CrawlerError::Fetch
        })?;
        let mut writer = BufWriter::new(file);

        serde_json::to_writer(&mut writer, &full_contents).map_err(|e| {
            error!("writing the content file failed: {e}");
            CrawlerError::Fetch
        })?;

        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_link_extraction_valid_html_link() {
        let link = String::from("<a href=\"https://this-week-in-rust.org/blog/2013/08/10/this-week-in-rust-10/\">This Week in Rust 10</a>");

        let currated_link = TwirCrawler::extract_link_and_title(link);
        assert_eq!(
            currated_link.link,
            Link(String::from(
                "https://this-week-in-rust.org/blog/2013/08/10/this-week-in-rust-10/"
            ))
        );
    }

    #[test]
    pub fn test_link_extraction_no_html() {
        let link =
            String::from("https://this-week-in-rust.org/blog/2013/08/10/this-week-in-rust-10/");

        let currated_link = TwirCrawler::extract_link_and_title(link);
        assert_eq!(
            currated_link.link,
            Link(String::from(
                "https://this-week-in-rust.org/blog/2013/08/10/this-week-in-rust-10/"
            ))
        );
    }

    #[tokio::test]
    pub async fn test_get_twir_issues() {
        let crawler = TwirCrawler::default();
        let issues = crawler.get_all_archived_twir_issues().await.unwrap();

        // not the greatest but it somehow validates that it received
        // the issues
        assert!(issues.len() > 500);
    }

    #[tokio::test]
    pub async fn test_filter_issues() {
        let crawler = TwirCrawler::default();
        let issue1 = TwirLinkElement::new(
            Link(String::from("https://www.google.com")),
            String::from("Google"),
        );
        let issue2 = TwirLinkElement::new(
            Link(String::from("https://www.youtube.com")),
            String::from("Youtube"),
        );

        let mut issues = vec![issue1.clone(), issue2];
        crawler.lychee_filter_issues(&mut issues).await;
        assert_eq!(issues.len(), 2);
    }
}
