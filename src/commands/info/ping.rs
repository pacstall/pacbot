use poise::serenity_prelude::*;

use crate::{Context, PoiseResult};

/// Get the bot's ping information
#[poise::command(slash_command, prefix_command, category = "Info")]
pub async fn ping(ctx: Context<'_>) -> PoiseResult {
    let before = std::time::SystemTime::now();
    let ping_msg = ctx.say("Calculating...").await?;

    let shard_manager = ctx.framework().shard_manager();

    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;

    let runner = runners
        .get(&ShardId(ctx.discord().shard_id))
        .ok_or("No shard found")?;

    ping_msg
        .edit(ctx, |b| {
            b.content("");
            b.embed(|msg| {
                let latency_msg = match runner.latency {
                    Some(duration) => {
                        format!(":heartbeat: {} ms", duration.as_millis())
                    },
                    None => ":ghost: No heartbeats yet!".into(),
                };

                let reply = format!(
                    ":clock2: {} ms\n{latency_msg}",
                    before.elapsed().unwrap().as_millis(),
                );
                msg.description(reply).color(Color::BLITZ_BLUE)
            })
        })
        .await?;

    Ok(())
}
