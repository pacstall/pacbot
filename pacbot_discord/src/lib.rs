#![allow(clippy::wildcard_imports)]
#![allow(clippy::missing_panics_doc)]

use std::env;
use std::time::Duration;

use dotenvy::dotenv;
use flume::{Receiver, Sender};
use libpacbot::{messages, Error};
use poise::serenity_prelude::{self as serenity, Activity};
use reqwest::Client;
use serenity::{GatewayIntents, GuildId};
use sysinfo::{System, SystemExt};
use tokio::sync::Mutex;

mod commands;

type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    client: Client,
    system_info: Mutex<System>,
}

#[allow(clippy::unreadable_literal)]
async fn on_ready(
    rx: Receiver<messages::Discord>,
    ctx: &serenity::Context,
    client: Client,
    _ready: &serenity::Ready,
    framework: &poise::Framework<Data, Error>,
) -> Result<Data, Error> {
    let pacstall_guild_id = GuildId(839818021207801878);

    let builder = poise::builtins::create_application_commands(&framework.options().commands);

    GuildId::set_application_commands(&pacstall_guild_id, &ctx.http, |commands| {
        *commands = builder.clone();

        commands
    })
    .await
    .expect("Error registering slash commands");

    tracing::info!("PacBot's online and ready to kick ass!");

    // Trigger status update cycle
    let status_rx = rx.clone();
    let status_ctx = ctx.clone();
    tokio::spawn(async move {
        loop {
            if let Ok(messages::Discord::StatusUpdate(status)) = status_rx.try_recv() {
                match status {
                    Some(status) => status_ctx.set_activity(Activity::watching(status)).await,
                    None => {
                        status_ctx
                            .set_activity(Activity::watching("for new pacscripts"))
                            .await;
                    },
                }
            };

            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    Ok(Data {
        client,
        system_info: Mutex::new(System::new()),
    })
}

pub async fn run(_tx: Sender<()>, rx: Receiver<messages::Discord>) {
    dotenv().expect("Unable to load .env!");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::repository::packagelist(),
                commands::repository::packageinfo(),
                commands::info::serverstats(),
                commands::info::about(),
                commands::info::ping(),
                commands::info::version(),
                commands::help(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(".".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .token(env::var("DISCORD_TOKEN").unwrap())
        .intents(GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT)
        .setup(|ctx, ready, framework| {
            let client = Client::builder()
                .user_agent("Pacstall-PacBot")
                .build()
                .unwrap();

            Box::pin(on_ready(rx, ctx, client, ready, framework))
        });

    framework.run().await.unwrap();
}
