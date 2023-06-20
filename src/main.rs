pub mod crawler;
pub mod logger;

use crawler::{get_latest_issue_index, parse_page};

use tracing::{error, info};

#[tokio::main]
async fn main() {
    logger::set_tracing();

    let links_and_titles = get_latest_issue_index().await;
    match links_and_titles {
        Ok(links_and_titles) => {
            links_and_titles
                .into_iter()
                .for_each(|(link, title)| info!("Obtained link: {} \t and title: {}", link, title));
        }
        Err(e) => {
            error!("Error occured while getting the links from twir: {e:?}");
            std::process::exit(-1);
        }
    }

    parse_page().await;
}
