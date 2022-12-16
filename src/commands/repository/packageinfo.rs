use poise::serenity_prelude::*;

use super::*;
use crate::commands::utility::fetch_packagelist;
use crate::{Context, PoiseResult};

async fn packagelist_autocomplete(ctx: Context<'_>, partial: &str) -> Vec<String> {
    fetch_packagelist(ctx)
        .await
        .into_iter()
        .filter(|s| s.starts_with(partial))
        .collect()
}

/// Get info about a package
#[allow(clippy::too_many_lines)]
#[poise::command(slash_command, prefix_command, category = "Repository")]
pub async fn packageinfo(
    ctx: Context<'_>,
    #[description = "Name of the package"]
    #[autocomplete = "packagelist_autocomplete"]
    name: String,
) -> PoiseResult {
    let client = &ctx.data().client;
    let response: ResponseData = match client
        .get(format!("https://pacstall.dev/api/packages/{name}"))
        .send()
        .await?
        .json()
        .await
    {
        Ok(json) => json,
        Err(error) => {
            let reason = {
                if error.is_decode() {
                    "Package not found!"
                } else {
                    "Unknown Error"
                }
            };
            ctx.send(|builder| {
                builder.embed(|msg| {
                    msg.color(Color::RED)
                        .title(reason)
                        .field("Details", error.to_string(), true)
                })
            })
            .await?;
            return Ok(());
        },
    };

    let color = match response.update_status {
        -1 => Color::DARK_GREY,
        0 => Color::GOLD,
        1 => Color::LIGHT_GREY,
        2 => Color::LIGHTER_GREY,
        3 => Color::RED,
        _ => unreachable!(),
    };

    let mut fields = vec![
        ("Name", format!("`{}`", response.name), true),
        ("Package Name", format!("`{}`", response.package_name), true),
        ("Version", response.version, true),
    ];

    if let Some(latest_version) = response.latest_version {
        if response.update_status != 0 {
            fields.push(("Latest Version", latest_version, true));
        }
    }
    fields.push(("Payload", format!("[Download]({})", response.url), true));
    if !response.runtime_dependencies.is_empty() {
        fields.push((
            "Runtime Dependencies",
            response
                .runtime_dependencies
                .into_iter()
                .map(|dependency| format!("`{dependency}`\n"))
                .collect::<String>(),
            true,
        ));
    }
    if !response.build_dependencies.is_empty() {
        fields.push((
            "Build Dependencies",
            response
                .build_dependencies
                .into_iter()
                .map(|dependency| format!("`{dependency}`\n"))
                .collect::<String>(),
            true,
        ));
    }
    if !response.optional_dependencies.is_empty() {
        fields.push((
            "Optional Dependencies",
            response
                .optional_dependencies
                .into_iter()
                .map(|dependency| format!("`{dependency}`\n"))
                .collect::<String>(),
            true,
        ));
    }
    if !response.pacstall_dependencies.is_empty() {
        fields.push((
            "Pacstall Dependencies",
            response
                .pacstall_dependencies
                .into_iter()
                .map(|dependency| format!("`{dependency}`\n"))
                .collect::<String>(),
            true,
        ));
    }
    if !response.breaks.is_empty() {
        fields.push((
            "Breaks",
            response
                .breaks
                .into_iter()
                .map(|pkgname| format!("`{pkgname}`\n"))
                .collect::<String>(),
            true,
        ));
    }
    if !response.gives.is_empty() {
        fields.push(("Gives", format!("`{}`", response.gives), true));
    }
    if !response.replace.is_empty() {
        fields.push((
            "Replaces",
            response
                .replace
                .into_iter()
                .map(|replace| format!("`{replace}`\n"))
                .collect::<String>(),
            true,
        ));
    }
    if !response.required_by.is_empty() {
        fields.push((
            "Required By",
            response
                .required_by
                .into_iter()
                .map(|requirement| format!("`{requirement}`\n"))
                .collect::<String>(),
            true,
        ));
    }
    if !response.ppa.is_empty() {
        fields.push((
            "PPA",
            response
                .ppa
                .into_iter()
                .map(|ppa| format!("`{ppa}`\n"))
                .collect::<String>(),
            true,
        ));
    }
    if !response.repology.is_empty() {
        fields.push((
            "Repology",
            response
                .repology
                .into_iter()
                .map(|repology| {
                    let split: Vec<&str> = repology.split(": ").collect();
                    format!("`{}`: `{}`\n", split[0], split[1])
                })
                .collect::<String>(),
            true,
        ));
    }

    let mut author = CreateEmbedAuthor::default();

    if response.maintainer.contains('<') {
        let split: Vec<&str> = response.maintainer.split('<').collect();
        let name = split[0].trim();

        let author = author.name(name);

        if client
            .get(format!("https://github.com/{name}"))
            .send()
            .await?
            .status()
            .is_success()
        {
            author
                .url(format!("https://github.com/{name}"))
                .icon_url(format!("https://github.com/{name}.png"));
        }
    } else {
        author.name(response.maintainer);
    }

    ctx.send(|builder| {
        builder.embed(|msg| {
            msg.title(response.pretty_name)
                .url(format!("https://github.com/pacstall/pacstall-programs/blob/master/packages/{}/{}.pacscript", response.name, response.name))
                .color(color)
                .description(response.description)
                .fields(fields)
                .set_author(author)
        })
    })
    .await?;

    Ok(())
}
