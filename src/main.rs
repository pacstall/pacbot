#![allow(clippy::wildcard_imports)]

use std::env;

use dotenvy::dotenv;
use parking_lot::Mutex;
use poise::serenity_prelude as serenity;
use serenity::{GatewayIntents, GuildId};
use sysinfo::{System, SystemExt};

mod commands;

type Error = Box<dyn std::error::Error + Send + Sync>;
type PoiseResult = Result<(), Error>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    system_info: Mutex<System>,
}

async fn on_ready(
    ctx: &serenity::Context,
    _ready: &serenity::Ready,
    framework: &poise::Framework<Data, Error>,
) -> Result<Data, Error> {
    let pacstall_guild_id: u64 = env::var("PACSTALL_GUILDID")?.parse::<u64>()?;

    let builder = poise::builtins::create_application_commands(&framework.options().commands);

    GuildId::set_application_commands(&GuildId(pacstall_guild_id), &ctx.http, |commands| {
        *commands = builder.clone();

        commands
    })
    .await
    .expect("Error registering slash commands");

    tracing::info!("PacBot's online and ready to kick ass!");

    Ok(Data {
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
                commands::info::about(),
                commands::info::ping(),
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
