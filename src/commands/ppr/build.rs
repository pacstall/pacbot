use jwt_simple::prelude::{Claims, Duration, RS256KeyPair, RSAKeyPairLike};
use rand::Rng;
use serde::Deserialize;
use serde_json::json;
use tokio::fs;
use tokio::time::sleep;

use crate::commands::utility::fetch_packagelist;
use crate::{Context, Error, PoiseResult};

#[derive(Deserialize, Debug)]
struct GitHubJobSteps {
    name: String,
}

#[derive(Deserialize, Debug)]
struct GitHubJob {
    steps: Vec<GitHubJobSteps>,
    html_url: String,
}

#[derive(Deserialize, Debug)]
struct GitHubJobsResponse {
    jobs: Vec<GitHubJob>,
}

#[derive(Deserialize, Debug)]
struct GitHubWorkflowRunsResponse {
    jobs_url: String,
}

#[derive(Deserialize, Debug)]
struct GitHubActionsListResponse {
    workflow_runs: Vec<GitHubWorkflowRunsResponse>,
}

#[derive(Deserialize, Debug)]
struct GitHubTokenResponse {
    token: String,
}

async fn packagelist_autocomplete(ctx: Context<'_>, partial: &str) -> Vec<String> {
    fetch_packagelist(ctx)
        .await
        .into_iter()
        .filter(|s| s.starts_with(partial) && !s.ends_with("-deb"))
        .collect()
}

async fn check_is_dev(ctx: Context<'_>) -> Result<bool, Error> {
    Ok(
        if ctx
            .author()
            .has_role(
                &ctx.discord().http,
                ctx.data().guild_id,
                ctx.data().dev_roll_id,
            )
            .await?
        {
            true
        } else {
            ctx.send(|builder| {
                builder
                    .ephemeral(true)
                    .content("This command is only available to the Pacstall Devs")
            })
            .await?;
            false
        },
    )
}

/// Trigger a new build.
#[poise::command(
    slash_command,
    prefix_command,
    category = "PPR",
    check = "check_is_dev"
)]
pub async fn build(
    ctx: Context<'_>,
    #[description = "Name of the package you want to build"]
    #[autocomplete = "packagelist_autocomplete"]
    name: String,
) -> PoiseResult {
    if !fetch_packagelist(ctx)
        .await
        .into_iter()
        .filter(|s| !s.ends_with("-deb"))
        .any(|s| s == name)
    {
        ctx.say(format!("`{name}` does not exist!")).await?;
        return Ok(());
    }

    let msg = ctx
        .say(format!(
            "<a:loading:1039364510248620174> Dispatching build workflow for `{name}`"
        ))
        .await?;

    let key = RS256KeyPair::from_pem(&fs::read_to_string("pacbot.pem").await?)?;
    let claims = Claims::create(Duration::from_mins(10)).with_issuer("258575");
    let jwt_token = key.sign(claims)?;

    let client = &ctx.data().client;

    let github_token = client
        .post("https://api.github.com/app/installations/30964287/access_tokens")
        .header("accept", "application/vnd.github+json")
        .bearer_auth(&jwt_token)
        .send()
        .await?
        .json::<GitHubTokenResponse>()
        .await?
        .token;

    let run_identifier = rand::thread_rng().gen::<u64>().to_string();

    client.post(
        "https://api.github.com/repos/pacstall/chaotic-ppr/actions/workflows/39759909/dispatches",
    ).header("accept", "application/vnd.github+json")
     .header("Authorization", format!("token {github_token}"))
        .body(json!({
            "ref": "master",
            "inputs": {
                "id": run_identifier,
                "PackageName": name
            }
        }).to_string())
     .send().await?.error_for_status()?;

    msg.edit(ctx, |reply| {
        reply.content(format!(
            "<a:loading:1039364510248620174> Successfully dispatched build workflow for `{name}`! \
             Waiting for run ID..."
        ))
    })
    .await?;

    let run_date_filter = (chrono::Utc::now() - chrono::Duration::minutes(2)).to_rfc3339();
    let mut build_job_html_url = None;

    while build_job_html_url.is_none() {
        let response = client.get(format!("https://api.github.com/repos/pacstall/chaotic-ppr/actions/runs?created=>{run_date_filter}"))
                             .header("Authorization", format!("token {github_token}"))
                             .send().await?.json::<GitHubActionsListResponse>().await?;

        let runs = response.workflow_runs;

        if runs.is_empty() {
            tracing::debug!("Waiting for workflows to popup...");
            sleep(std::time::Duration::from_secs(3)).await;
        } else {
            for workflow in runs {
                let jobs_url = workflow.jobs_url;

                let response = client
                    .get(jobs_url)
                    .header("Authorization", format!("token {github_token}"))
                    .send()
                    .await?
                    .json::<GitHubJobsResponse>()
                    .await?;

                let jobs = response.jobs;

                if jobs.is_empty() {
                    tracing::debug!("Waiting for jobs to popup...");
                    sleep(std::time::Duration::from_secs(3)).await;
                } else {
                    let id_job = &jobs[0];

                    let steps = &id_job.steps;

                    if steps.len() >= 2 {
                        let second_step = &steps[1];

                        tracing::debug!("{second_step:#?} => {run_identifier}");

                        if second_step.name == run_identifier {
                            let build_job = &jobs[1];
                            build_job_html_url = Some(build_job.html_url.clone());
                        }
                    } else {
                        tracing::debug!("Waiting for steps to be executed...");
                        sleep(std::time::Duration::from_secs(3)).await;
                    }
                }
            }
        }
    }

    msg.edit(ctx, |reply| {
        reply.content(format!(
            ":white_check_mark: Successfully dispatched build workflow for `{name}`! Here's the \
             job link: {}",
            build_job_html_url.unwrap()
        ))
    })
    .await?;

    Ok(())
}
