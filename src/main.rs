#![feature(once_cell)]
#![feature(map_try_insert)]
extern crate lazy_static;
extern crate regex;

use actix_web::{App, HttpServer, web};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use untitled::entrypoint::post_github_push_event_handler::handle_post_github_push_event;
use untitled::InitError;

#[tokio::main]
async fn main() -> Result<(), InitError> {

    /*let client = Client::builder().build();

    let github_user_info = client
        .get("https://api.github.com/user")
        .header("User-Agent", "request")
        .header("accept", "*")
        .bearer_auth("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")
        .send()
        .await
        .unwrap()
        .json::<GithubUserInfo>()
        .await
        .unwrap();

    println!("{}", github_user_info.name);*/

    /*let a = reqwest::get("https://raw.githubusercontent.com/romqu/ubuntu-config-scripts/master/README.md?token=GHSAT0AAAAAABR6TMBHGSVKHW4HAVLNYJIYQZGTZA")
    .await.unwrap().status();*/

    /*    match init_app() {
        Ok(_) => start_app().await.map_err(|_| CouldNotStartApp),
        Err(err) => Err(err),
    }
*/

    Ok({})
}

pub async fn start_app() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route(
            "/api/v1/events/push",
            web::post().to(handle_post_github_push_event),
        )
    })
        .bind("0.0.0.0:8083")?
        .run()
        .await
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubUserInfo {
    pub login: String,
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    #[serde(rename = "gravatar_id")]
    pub gravatar_id: String,
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "followers_url")]
    pub followers_url: String,
    #[serde(rename = "following_url")]
    pub following_url: String,
    #[serde(rename = "gists_url")]
    pub gists_url: String,
    #[serde(rename = "starred_url")]
    pub starred_url: String,
    #[serde(rename = "subscriptions_url")]
    pub subscriptions_url: String,
    #[serde(rename = "organizations_url")]
    pub organizations_url: String,
    #[serde(rename = "repos_url")]
    pub repos_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "received_events_url")]
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "site_admin")]
    pub site_admin: bool,
    pub name: String,
    pub company: Value,
    pub blog: String,
    pub location: String,
    pub email: Value,
    pub hireable: bool,
    pub bio: String,
    #[serde(rename = "twitter_username")]
    pub twitter_username: Value,
    #[serde(rename = "public_repos")]
    pub public_repos: u64,
    #[serde(rename = "public_gists")]
    pub public_gists: i64,
    pub followers: i64,
    pub following: i64,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "private_gists")]
    pub private_gists: i64,
    #[serde(rename = "total_private_repos")]
    pub total_private_repos: u64,
    #[serde(rename = "owned_private_repos")]
    pub owned_private_repos: u64,
    #[serde(rename = "disk_usage")]
    pub disk_usage: i64,
    pub collaborators: i64,
    #[serde(rename = "two_factor_authentication")]
    pub two_factor_authentication: bool,
    pub plan: Plan,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plan {
    pub name: String,
    pub space: i64,
    pub collaborators: i64,
    #[serde(rename = "private_repos")]
    pub private_repos: i64,
}
