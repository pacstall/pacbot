#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]

use std::time::Duration;

use flume::{Receiver, Sender};
use libpacbot::messages;
use reqwest::Client;
use tokio::time::interval;

use crate::issue::manage_issues_for_outdated_pacscripts;

mod graphql;
mod issue;

pub async fn run(tx: Sender<messages::Discord>, _rx: Receiver<()>) {
    tracing::info!("Started GitHub bot");

    let client = Client::builder()
        .user_agent("Pacstall-PacBot")
        .build()
        .unwrap();

    let mut timer = interval(Duration::from_secs(900)); // 900 seconds = 15 minute
    tokio::spawn(async move {
        loop {
            timer.tick().await;
            tracing::info!("Starting to manage issues");
            tx.send_async(messages::Discord::StatusUpdate(Some(
                "issues refresh".to_owned(),
            )))
            .await
            .unwrap();
            manage_issues_for_outdated_pacscripts(&client)
                .await
                .unwrap();
            tx.send_async(messages::Discord::StatusUpdate(None))
                .await
                .unwrap();
            tracing::info!("Finished managing issues");
        }
    })
    .await
    .unwrap();
}
