use poise::serenity_prelude::*;
use sysinfo::{ProcessExt, SystemExt};

use crate::{Context, PoiseResult};

/// Shows information about this bot
#[poise::command(slash_command, prefix_command, category = "Info")]
pub async fn about(ctx: Context<'_>) -> PoiseResult {
    let ram_usage = {
        let mut system_info = ctx.data().system_info.lock();
        system_info.refresh_specifics(
            sysinfo::RefreshKind::new()
                .with_cpu(sysinfo::CpuRefreshKind::new().with_cpu_usage())
                .with_processes(sysinfo::ProcessRefreshKind::new())
                .with_memory(),
        );

        let pid = sysinfo::get_current_pid().unwrap();
        system_info.process(pid).unwrap().memory() / 1024 / 1024
    };

    ctx.send(|builder| {
        builder.embed(|msg| {
            msg.title("Bot Information")
                .thumbnail(ctx.discord().cache.current_user().avatar_url().unwrap())
                .description("A bot to assist Pacstall Devs, written in Rust! :crab:")
                .fields([
                    ("Version", env!("CARGO_PKG_VERSION"), true),
                    ("RAM Usage", &format!("{ram_usage}MB of RAM"), true),
                ])
                .color(Color::GOLD)
        })
    })
    .await?;

    Ok(())
}
