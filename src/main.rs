pub mod crawler;
pub mod logger;

use crawler::get_latest_issue_index;

use tracing::{error, info};

#[tokio::main]
async fn main() {
    logger::set_tracing();

    let links = get_latest_issue_index().await;
    match links {
        Ok(links) => {
            info!("Obtained {} this week in rust issue links", links.len());
        }
        Err(e) => {
            error!("Error occured while getting the links from twir: {e:?}");
            std::process::exit(-1);
        }
    }
}
