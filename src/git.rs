use octocrab::{self, Page};
use tokio;

pub async fn get_something() -> Page<octocrab::models::issues::Issue> {
    let octocrab = octocrab::instance();

    // Print the titles of the first page of issues.
    octocrab
        .issues("rust-lang", "rust")
        .list()
        .send()
        .await
        .unwrap()
}
