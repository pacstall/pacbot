use jwt_simple::prelude::*;
use reqwest::Client;
use serde::Deserialize;
use tokio::fs;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Deserialize, Debug)]
struct GitHubTokenResponse {
    token: String,
}

/// Gets a token from GitHub for use in API requests.
///
/// # Errors
///
/// if there was a problem creating the JWT token, reading the private key file,
/// making the HTTP request, or parsing the response.
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
