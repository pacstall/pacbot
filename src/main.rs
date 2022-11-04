#![allow(clippy::wildcard_imports)]

use dotenv_codegen::dotenv;
use poise::serenity_prelude as serenity;
use serenity::{GatewayIntents, GuildId};

mod commands;

type Error = Box<dyn std::error::Error + Send + Sync>;
type PoiseResult = Result<(), Error>;
type Context<'a> = poise::Context<'a, Data, Error>;
// User data, which is stored and accessible in all command invocations
pub struct Data {}

async fn on_ready(
    ctx: &serenity::Context,
    _ready: &serenity::Ready,
    framework: &poise::Framework<Data, Error>,
) -> Result<Data, Error> {
    let pacstall_guild_id: u64 = dotenv!("PACSTALL_GUILDID")
        .to_string()
        .parse::<u64>()
        .unwrap();

    let builder = poise::builtins::create_application_commands(&framework.options().commands);

    let commands = serenity::GuildId::set_application_commands(
        &GuildId(pacstall_guild_id),
        &ctx.http,
        |commands| {
            *commands = builder.clone();

            commands
        },
    )
    .await;

    tracing::info!("Following slash commands registered:\n{commands:#?}");

    Ok(Data {})
}

#[tokio::main]
async fn main() {
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
        .token(dotenv!("DISCORD_TOKEN"))
        .intents(GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT)
        .user_data_setup(|ctx, ready, framework| Box::pin(on_ready(ctx, ready, framework)));

    framework.run().await.unwrap();
}
