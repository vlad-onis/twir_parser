pub mod cli;
pub mod logger;
pub mod web_crawler;

use web_crawler::twir_crawler::TwirCrawler;

use tracing::{error, info};

#[tokio::main]
async fn main() {
    logger::set_tracing();
    let search_word = cli::get_search_arg();

    let crawler = TwirCrawler::new();
    crawler.search(search_word).await;
}
