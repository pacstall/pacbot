use std::time::Duration;

use libpacbot::website::PackagesResponse;
use libpacbot::PacResult;
use poise::futures_util::StreamExt;
use poise::serenity_prelude::*;

use crate::Context;

/// Get the package list
#[poise::command(slash_command, prefix_command, category = "Repository")]
pub async fn packagelist(ctx: Context<'_>) -> PacResult {
    let response: PackagesResponse = ctx
        .data()
        .client
        .get("https://pacstall.dev/api/packages?page=0")
        .send()
        .await?
        .json()
        .await?;

    let mut links = String::with_capacity(4000);
    let mut count = 0;
    for data in response.data {
        let name = data.name;
        let link = &format!("[{name}](https://github.com/pacstall/pacstall-programs/blob/master/packages/{name}/{name}.pacscript)\n");

        if count > 4000 {
            break;
        }

        count += link.len();
        links.push_str(link);
    }

    tracing::debug!("{links}");

    let reply = ctx
        .send(|builder| {
            builder
                .embed(|msg| {
                    msg.title("Package List")
                        .color(Color::GOLD)
                        .url("https://github.com/pacstall/pacstall-programs/blob/master/packages")
                        .description(&links)
                        .footer(|footer| footer.text(format!("Total packages: {}", response.total)))
                })
                .components(|component| {
                    component.create_action_row(|row| {
                        row.create_button(|btn| {
                            btn.style(ButtonStyle::Secondary)
                                .custom_id("back")
                                .label("Back")
                        });
                        row.create_button(|btn| {
                            btn.style(ButtonStyle::Primary)
                                .custom_id("next")
                                .label("Next")
                        })
                    })
                })
        })
        .await?;

    while let Some(interaction) = reply
        .message()
        .await?
        .await_component_interactions(&ctx.serenity_context().shard)
        .timeout(Duration::from_secs(60 * 5))
        .author_id(ctx.author().id)
        .build()
        .next()
        .await
    {
        match &*interaction.data.custom_id {
            "next" => {
                interaction
                    .create_interaction_response(&ctx.serenity_context().http, |resp| {
                        resp.kind(InteractionResponseType::UpdateMessage)
                            .interaction_response_data(|data| {
                                data.embed(|msg| msg.description("test"))
                            })
                    })
                    .await?;
            },
            // "back" => ctx.say("Back clicked!").await?,
            _ => unreachable!(),
        };
    }

    Ok(())
}
