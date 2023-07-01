pub mod cli;
pub mod logger;
pub mod model;
pub mod web_crawler;

use clap::Parser;
use web_crawler::twir_crawler::TwirCrawler;

#[tokio::main]
async fn main() {
    logger::set_tracing();

    let args = cli::Args::parse();

    let search_sentence = args.search;
    let search_online = args.online;
    let limit = args.limit;

    let crawler = TwirCrawler::default();
    crawler.search(search_sentence, search_online, limit).await;
}
