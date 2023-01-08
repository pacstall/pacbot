pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

use libpacbot::PacResult;
use poise::serenity_prelude::*;

use crate::Context;

/// Shows a detailed summary of the bot's version
#[poise::command(slash_command, prefix_command, category = "Info")]
pub async fn version(ctx: Context<'_>) -> PacResult {
    ctx.send(|builder| {
        builder.embed(|msg| {
            msg.title("Version Information")
                .thumbnail(
                    ctx.serenity_context()
                        .cache
                        .current_user()
                        .avatar_url()
                        .unwrap(),
                )
                .fields([
                    (
                        "Bot Version",
                        format!(
                            "[{}](https://github.com/pacstall/pacbot/commit/{})",
                            &built_info::GIT_COMMIT_HASH.unwrap()[..7],
                            built_info::GIT_COMMIT_HASH.unwrap()
                        ),
                        true,
                    ),
                    (
                        "Rust Version",
                        format!("`{}`", built_info::RUSTC_VERSION),
                        true,
                    ),
                    ("Profile", built_info::PROFILE.into(), true),
                    (
                        "Built At",
                        format!(
                            "<t:{}:F>",
                            built::util::strptime(built_info::BUILT_TIME_UTC).timestamp()
                        ),
                        true,
                    ),
                ])
                .color(Color::GOLD)
        })
    })
    .await?;

    Ok(())
}
