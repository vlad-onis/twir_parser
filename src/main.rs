pub mod cli;
pub mod logger;
pub mod model;
pub mod web_crawler;

use web_crawler::twir_crawler::TwirCrawler;

#[tokio::main]
async fn main() {
    logger::set_tracing();
    let search_word = cli::get_search_arg();

    let crawler = TwirCrawler::new();
    crawler.search(search_word).await;
}
