use jwt_simple::prelude::*;
use reqwest::Client;
use serde::Deserialize;
use tokio::fs;

use crate::Error;

#[derive(Deserialize, Debug)]
struct GitHubTokenResponse {
    token: String,
}

pub async fn get_github_token(client: &Client) -> Result<String, Error> {
    let key = RS256KeyPair::from_pem(&fs::read_to_string("pacbot.pem").await?)?;
    let claims = Claims::create(Duration::from_mins(10)).with_issuer("258575");
    let jwt_token = key.sign(claims)?;

    Ok(client
        .post("https://api.github.com/app/installations/30964287/access_tokens")
        .header("accept", "application/vnd.github+json")
        .bearer_auth(&jwt_token)
        .send()
        .await?
        .json::<GitHubTokenResponse>()
        .await?
        .token)
}
