use crate::Context;

pub async fn fetch_packagelist(ctx: Context<'_>) -> Vec<String> {
    ctx.data()
        .client
        .get("https://raw.githubusercontent.com/pacstall/pacstall-programs/master/packagelist")
        .send()
        .await
        .expect("Error when sending the request to fetch packagelist")
        .error_for_status()
        .expect("Status error when requesting the packagelist")
        .text()
        .await
        .expect("Error when getting the response text of the packagelist request")
        .lines()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
}
