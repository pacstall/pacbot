use libpacbot::PacResult;

#[tokio::main]
async fn main() -> PacResult {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let (discord_tx, discord_rx) = flume::bounded(5);
    let (github_tx, github_rx) = flume::bounded(5);

    tokio::join!(
        pacbot_discord::run(github_tx, discord_rx),
        pacbot_github::run(discord_tx, github_rx)
    );
    Ok(())
}
