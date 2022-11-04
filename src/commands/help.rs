use poise::builtins;

use crate::{Context, PoiseResult};

/// Show a help menu
#[poise::command(slash_command, prefix_command, category = "Info")]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Command to display specific information about"] command: Option<String>,
) -> PoiseResult {
    builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration::default(),
    )
    .await?;

    Ok(())
}
