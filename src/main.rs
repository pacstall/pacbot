#![allow(clippy::wildcard_imports)]

use std::env;

use dotenvy::dotenv;
use poise::serenity_prelude::{self as serenity, RoleId};
use reqwest::Client;
use serenity::{GatewayIntents, GuildId};
use sysinfo::{System, SystemExt};
use tokio::sync::Mutex;

mod commands;

type Error = Box<dyn std::error::Error + Send + Sync>;
type PoiseResult = Result<(), Error>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    client: Client,
    guild_id: GuildId,
    dev_roll_id: RoleId,
    system_info: Mutex<System>,
}

#[allow(clippy::unreadable_literal)]
async fn on_ready(
    ctx: &serenity::Context,
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

    Ok(Data {
        client: Client::builder()
            .user_agent("Pacstall-PacBot")
            .build()
            .unwrap(),
        guild_id: pacstall_guild_id,
        dev_roll_id: RoleId(839834742471655434),
        system_info: Mutex::new(System::new()),
    })
}

#[tokio::main]
async fn main() {
    dotenv().expect("Unable to load .env!");

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::repository::packagelist(),
                commands::repository::packageinfo(),
                commands::ppr::ppr(),
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
        .user_data_setup(|ctx, ready, framework| Box::pin(on_ready(ctx, ready, framework)));

    framework.run().await.unwrap();
}
