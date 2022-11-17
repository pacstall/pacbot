use crate::{Context, PoiseResult};

pub mod build;

pub use build::build;

/// Control the Pacstall Package Repository
#[poise::command(slash_command, prefix_command, category = "PPR", subcommands("build"))]
pub async fn ppr(ctx: Context<'_>) -> PoiseResult {
    ctx.say("Please use a subcommand").await?;
    Ok(())
}
