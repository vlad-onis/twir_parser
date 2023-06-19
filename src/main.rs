pub mod crawler;

use crawler::get_latest_issue_index;

#[tokio::main]
async fn main() {
    let links = get_latest_issue_index().await;
    println!("{links:?}");
}
