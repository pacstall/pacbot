use poise::serenity_prelude::*;

use crate::{Context, PoiseResult};

/// Shows information about this bot
#[poise::command(slash_command, prefix_command, category = "Info")]
pub async fn about(ctx: Context<'_>) -> PoiseResult {
    ctx.send(|builder| {
        builder.embed(|msg| {
            msg.title("Bot Information")
                .thumbnail(ctx.discord().cache.current_user().avatar_url().unwrap())
                .description("A bot to assist Pacstall Devs, written in Rust! :crab:")
                .field("Version", env!("CARGO_PKG_VERSION"), true)
                .color(Color::GOLD)
        })
    })
    .await?;

    Ok(())
}
