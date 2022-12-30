use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

use super::utils::get_github_token;
use crate::commands::repository::PackagesResponse;
use crate::PoiseResult;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
enum StateReason {
    Completed,
    NotPlanned,
}

#[derive(Deserialize, Debug)]
struct GitHubIssue {
    number: u32,
    title: String,
    state_reason: Option<StateReason>,
}

#[allow(clippy::too_many_lines)]
pub async fn manage_issues_for_outdated_pacscripts(client: &Client) -> PoiseResult {
    tracing::trace!("Getting github token");
    let github_token = get_github_token(client).await?;

    // Get list of open issues created by the bot
    tracing::trace!("Getting list of open issues");

    // let issues = client
    //     .get("https://api.github.com/repos/pacstall/pacstall-programs/issues")
    //     .query(&[
    //         ("per_page", "100"),
    //         ("state", "open"),
    //         ("creator", "app/pacstall-pacbot"),
    //     ])
    //     .send()
    //     .await?
    //     .text()
    //     .awai
    //
    // let issues = issues.headers();
    //
    // tracing::debug!("{issues:?}");

    let github_open_issues: Vec<GitHubIssue> = client
        .get("https://api.github.com/repos/pacstall/pacstall-programs/issues")
        .query(&[
            ("per_page", "100"),
            ("state", "open"),
            ("creator", "app/pacstall-pacbot"),
        ])
        .send()
        .await?
        .json()
        .await?;

    // Get list of closed issues created by the bot
    tracing::trace!("Getting list of closed issues");
    let mut github_closed_issues: Vec<GitHubIssue> = vec![];

    let mut page_count = 1;
    loop {
        let response = client
            .get("https://api.github.com/repos/pacstall/pacstall-programs/issues")
            .query(&[
                ("page", page_count.to_string()),
                ("per_page", "100".to_owned()),
                ("state", "closed".to_owned()),
                ("creator", "app/pacstall-pacbot".to_owned()),
            ])
            .send()
            .await?
            .json::<Vec<GitHubIssue>>()
            .await?;

        if response.is_empty() {
            break;
        }
        for closed_issue in response {
            if closed_issue.state_reason.as_ref().unwrap() == &StateReason::NotPlanned {
                github_closed_issues.push(closed_issue);
            }
        }
        page_count += 1;
    }

    tracing::debug!("{github_closed_issues:?}");

    // HACK: `pageSize` argument doesn't work currently
    // Therefore gather all the packages' data by looping
    let initial_packages_response: PackagesResponse = client
        .get("https://pacstall.dev/api/packages")
        .query(&[("page", 0)])
        .send()
        .await?
        .json()
        .await?;

    let mut package_responses = vec![initial_packages_response.data];

    for idx in 1..=initial_packages_response.last_page {
        package_responses.push(
            client
                .get("https://pacstall.dev/api/packages".to_string())
                .query(&[("page", idx)])
                .send()
                .await?
                .json::<PackagesResponse>()
                .await?
                .data,
        );
    }

    tracing::debug!("GitHub open issues: {github_open_issues:?}");
    tracing::debug!("Package responses: {package_responses:?}");

    for response in package_responses {
        for package in response {
            // If update status isn't unknown
            if package.update_status != -1 {
                let name = package.name;

                // If update status isn't latest
                if package.update_status > 0 {
                    let version = package.version;
                    let latest_version = package.latest_version.unwrap();
                    let title = format!("Outdated `{name}`: `{version}` -> `{latest_version}`");

                    // Only create issue if one with the same title doesn't already exist
                    if !github_open_issues.iter().any(|issue| issue.title == title)
                        && !github_closed_issues
                            .iter()
                            .any(|issue| issue.title == title)
                    {
                        let maintainer = package.maintainer;
                        let maintainer_split: Vec<&str> = maintainer.split('<').collect();

                        let maintainer_name = if maintainer_split.len() != 2 {
                            // There's no email address present, so the whole string is the
                            // maintainer's name
                            Some(maintainer.trim().to_lowercase())
                        } else if !maintainer_split[0].is_empty() {
                            // Use the first element from the split, i.e, the maintainer's name
                            Some(maintainer_split[0].trim().to_lowercase())
                        } else {
                            // There's no maintainer
                            None
                        };

                        let pretty_name = package.pretty_name;
                        let label = match package.update_status {
                            1 => "patch update",
                            2 => "minor update",
                            3 => "major update",
                            _ => unreachable!("Pacstall API broke"),
                        };

                        tracing::info!("Creating issue for {title}");

                        // Create the issue
                        if client
                        .post("https://api.github.com/repos/pacstall/pacstall-programs/issues")
                        .header("accept", "application/vnd.github+json")
                        .header("Authorization", format!("token {github_token}"))
                        .body(
                            // If there's a maintainer
                            (if let Some(username) = maintainer_name {
                                let raw_body = format!(
                                    r"
                                    |Field|Value|
                                    |-|:-:|
                                    |Pretty Name|{pretty_name}|
                                    |Name|`{name}`|
                                    |Current Version|`{version}`|
                                    |Latest Version|`{latest_version}`|
                                    |Maintainer|`{maintainer}`|

                                    > **Warning**
                                    > This should not be closed manually, or by using the *fixes/closes* keywords in your PR. This bot will take care of that for you."
                                );

                                // Remove the extra padding
                                let body = raw_body
                                    .lines()
                                    .map(|line| line.trim_matches(|c| c == ' ').to_owned() + "\n")
                                    .collect::<String>();

                                // Check if the maintainer's name is a valid username on github
                                if client
                                    .get(format!("https://github.com/{username}"))
                                    .send()
                                    .await?
                                    .error_for_status()
                                    .is_ok()
                                {
                                    json!({
                                        "title": title,
                                        "body": body,
                                        "labels": vec![label],
                                        "assignees": vec![username],
                                    })
                                } else {
                                    tracing::warn!(
                                        "{username} is not a valid GitHub username, so not \
                                         assigning them"
                                    );
                                    json!({
                                        "title": title,
                                        "body": body,
                                        "labels": vec![label],
                                    })
                                }
                            } else {
                                json!({
                                    "title": title,
                                    "body": format!(
                                    r"
                                    |Field|Value|
                                    |-|:-:|
                                    |Pretty Name|{pretty_name}|
                                    |Name|`{name}`|
                                    |Current Version|`{version}`|
                                    |Latest Version|`{latest_version}`|

                                    > **Warning**
                                    > This should not be closed manually, or by using the *fixes/closes* keywords in your PR. This bot will take care of that for you.")
                                    .lines()
                                    .map(|line| line.trim_matches(|c| c == ' ').to_owned() + "\n")
                                    .collect::<String>(),
                                    "labels": vec![label],
                                })
                            })
                            .to_string(),
                        )
                        .send()
                        .await?
                        .error_for_status()
                        .is_err()
                    {
                        tracing::warn!(
                            "{name}'s maintainer is not in our organization, so assigning them \
                             failed"
                        );
                    }
                    }
                }

                if package.update_status == 0 {
                    for issue in github_open_issues.iter().filter(|issue| {
                        // Find the first ` character
                        let start_index = issue.title.find('`').unwrap();

                        // Find the index of the second ` character, starting from the index of the
                        // first ` character
                        let end_index = issue.title[start_index + 1..].find('`').unwrap();

                        // Extract the string between the two ` characters, this is the package
                        // name of the issue
                        let substring = &issue.title[(start_index + 1)..=(start_index + end_index)];

                        substring == name
                    }) {
                        // Close the issue
                        if client
                            .post(format!(
                                "https://api.github.com/repos/pacstall/pacstall-programs/issues/{}",
                                issue.number
                            ))
                            .header("accept", "application/vnd.github+json")
                            .header("Authorization", format!("token {github_token}"))
                            .body(json!({"state": "close"}).to_string())
                            .send()
                            .await?
                            .error_for_status()
                            .is_ok()
                        {
                            tracing::info!(
                                "Closed issue number: {} as {name} is up-to-date",
                                issue.number
                            );
                        } else {
                            tracing::error!("Failed to close issue number: {}", issue.number);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
