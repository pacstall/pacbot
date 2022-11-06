use human_bytes::human_bytes;
use poise::serenity_prelude::*;
use sysinfo::{CpuExt, DiskExt, NetworkExt, NetworksExt, SystemExt};

use crate::{Context, PoiseResult};

/// Shows information the Pacstall server
#[allow(clippy::cast_precision_loss)]
#[poise::command(slash_command, prefix_command, category = "Info")]
pub async fn serverstats(ctx: Context<'_>) -> PoiseResult {
    let mut system_info = ctx.data().system_info.lock().await;
    system_info.refresh_networks_list();
    system_info.refresh_disks_list();
    system_info.refresh_all();

    let cpu_usage = system_info.global_cpu_info().cpu_usage();

    let used_memory = human_bytes(system_info.available_memory() as f64);
    let total_memory = human_bytes(system_info.total_memory() as f64);
    let ram_usage =
        (system_info.available_memory() as f32 / system_info.total_memory() as f32) * 100.0;

    let load_average = system_info.load_average();
    let one_min_load_avg = load_average.one;
    let five_min_load_avg = load_average.five;
    let fifteen_min_load_avg = load_average.fifteen;

    let os_version = system_info.long_os_version().unwrap();
    let kernel_version = system_info.kernel_version().unwrap();

    let network_activity: String = system_info
        .networks()
        .iter()
        .filter(|(_, data)| data.received() != 0 && data.transmitted() != 0)
        .map(|(interface_name, data)| {
            let data_received = human_bytes(data.received() as f32);
            let data_transmitted = human_bytes(data.transmitted() as f32 / 1024.0);

            format!("**[{interface_name}]**: {data_received}/s ↓ {data_transmitted}/s ↑\n")
        })
        .collect();

    let network_activity = if network_activity.is_empty() {
        "No network activity".into()
    } else {
        network_activity
    };

    let disk_usage: String = system_info
        .disks()
        .iter()
        .map(|disk| {
            format!(
                "**{}**: {} / {}\n",
                disk.mount_point().to_str().unwrap(),
                human_bytes((disk.total_space() - disk.available_space()) as f64),
                human_bytes(disk.total_space() as f64)
            )
        })
        .collect();

    let uptime = system_info.uptime();

    ctx.send(|builder| {
        builder.embed(|msg| {
            msg.title("Server Stats")
                .fields([
                    ("CPU Usage", format!("{cpu_usage:.1}%"), true),
                    (
                        "RAM Usage",
                        format!("{used_memory} / {total_memory} ({ram_usage:.1}%)"),
                        true,
                    ),
                    (
                        "Load Averages",
                        format!("{one_min_load_avg} {five_min_load_avg} {fifteen_min_load_avg}"),
                        true,
                    ),
                    ("Network Activity", network_activity, true),
                    ("Disk Usage", disk_usage, true),
                    ("Uptime", compound_duration::format_wdhms(uptime), true),
                ])
                .footer(|footer| footer.text(format!("OS: {os_version}\nKernel: {kernel_version}")))
                .color(Color::GOLD)
        })
    })
    .await?;

    Ok(())
}
