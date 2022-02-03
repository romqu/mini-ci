#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::borrow::Borrow;
use std::collections::HashMap;
use std::env;

use actix_web::web::Json;
use actix_web::{web, App, HttpResponse, HttpServer};
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::data::repo_info_repository::RepoInfoRepository;
use crate::deploy::clone_repo_service::CloneRepoService;
use crate::deploy::deploy_service::DeployService;
use crate::deploy::schimmelhof::deploy_schimmelhof_api_dev_service::DeploySchimmelhofApiDevService;

mod data;
mod deploy;

async fn index(item: Json<GithubPushEventDto>) -> HttpResponse {
    println!("model: {:?}", &item);
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Args = Args::parse();
    let repository = RepoInfoRepository::new(HashMap::new());

    let service = DeploySchimmelhofApiDevService::new(repository.clone());
    CloneRepoService::new(repository.clone()).execute(
        service.ssh_git_url(),
        "/tmp",
        "mini-ci",
        args.ssh_passphrase,
        args.ssh_key_path,
    );

    let dto = GithubPushEventDto::default();
    let dto1 = GithubPushEventDto {
        ref_field: "refs/heads/mvp".to_string(),
        ..dto
    };
    println!(
        "{}",
        service
            .execute(dto1)
            .unwrap()
            .to_string()
    );

    HttpServer::new(|| App::new().service(web::resource("/payload").route(web::post().to(index))))
        .bind("127.0.0.1:4567")?
        .run()
        .await
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    ssh_passphrase: String,

    #[clap(long)]
    ssh_key_path: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubPushEventDto {
    #[serde(rename = "ref")]
    pub ref_field: String,
    pub before: String,
    pub after: String,
    pub repository: Repository,
    pub pusher: Pusher,
    pub sender: Sender,
    pub created: bool,
    pub deleted: bool,
    pub forced: bool,
    #[serde(rename = "base_ref")]
    pub base_ref: Value,
    pub compare: String,
    pub commits: Vec<Commit>,
    #[serde(rename = "head_commit")]
    pub head_commit: HeadCommit,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    pub name: String,
    #[serde(rename = "full_name")]
    pub full_name: String,
    pub private: bool,
    pub owner: Owner,
    #[serde(rename = "html_url")]
    pub html_url: String,
    pub description: Value,
    pub fork: bool,
    pub url: String,
    #[serde(rename = "forks_url")]
    pub forks_url: String,
    #[serde(rename = "keys_url")]
    pub keys_url: String,
    #[serde(rename = "collaborators_url")]
    pub collaborators_url: String,
    #[serde(rename = "teams_url")]
    pub teams_url: String,
    #[serde(rename = "hooks_url")]
    pub hooks_url: String,
    #[serde(rename = "issue_events_url")]
    pub issue_events_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "assignees_url")]
    pub assignees_url: String,
    #[serde(rename = "branches_url")]
    pub branches_url: String,
    #[serde(rename = "tags_url")]
    pub tags_url: String,
    #[serde(rename = "blobs_url")]
    pub blobs_url: String,
    #[serde(rename = "git_tags_url")]
    pub git_tags_url: String,
    #[serde(rename = "git_refs_url")]
    pub git_refs_url: String,
    #[serde(rename = "trees_url")]
    pub trees_url: String,
    #[serde(rename = "statuses_url")]
    pub statuses_url: String,
    #[serde(rename = "languages_url")]
    pub languages_url: String,
    #[serde(rename = "stargazers_url")]
    pub stargazers_url: String,
    #[serde(rename = "contributors_url")]
    pub contributors_url: String,
    #[serde(rename = "subscribers_url")]
    pub subscribers_url: String,
    #[serde(rename = "subscription_url")]
    pub subscription_url: String,
    #[serde(rename = "commits_url")]
    pub commits_url: String,
    #[serde(rename = "git_commits_url")]
    pub git_commits_url: String,
    #[serde(rename = "comments_url")]
    pub comments_url: String,
    #[serde(rename = "issue_comment_url")]
    pub issue_comment_url: String,
    #[serde(rename = "contents_url")]
    pub contents_url: String,
    #[serde(rename = "compare_url")]
    pub compare_url: String,
    #[serde(rename = "merges_url")]
    pub merges_url: String,
    #[serde(rename = "archive_url")]
    pub archive_url: String,
    #[serde(rename = "downloads_url")]
    pub downloads_url: String,
    #[serde(rename = "issues_url")]
    pub issues_url: String,
    #[serde(rename = "pulls_url")]
    pub pulls_url: String,
    #[serde(rename = "milestones_url")]
    pub milestones_url: String,
    #[serde(rename = "notifications_url")]
    pub notifications_url: String,
    #[serde(rename = "labels_url")]
    pub labels_url: String,
    #[serde(rename = "releases_url")]
    pub releases_url: String,
    #[serde(rename = "deployments_url")]
    pub deployments_url: String,
    #[serde(rename = "created_at")]
    pub created_at: i64,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "pushed_at")]
    pub pushed_at: i64,
    #[serde(rename = "git_url")]
    pub git_url: String,
    #[serde(rename = "ssh_url")]
    pub ssh_url: String,
    #[serde(rename = "clone_url")]
    pub clone_url: String,
    #[serde(rename = "svn_url")]
    pub svn_url: String,
    pub homepage: Value,
    pub size: i64,
    #[serde(rename = "stargazers_count")]
    pub stargazers_count: i64,
    #[serde(rename = "watchers_count")]
    pub watchers_count: i64,
    pub language: Value,
    #[serde(rename = "has_issues")]
    pub has_issues: bool,
    #[serde(rename = "has_projects")]
    pub has_projects: bool,
    #[serde(rename = "has_downloads")]
    pub has_downloads: bool,
    #[serde(rename = "has_wiki")]
    pub has_wiki: bool,
    #[serde(rename = "has_pages")]
    pub has_pages: bool,
    #[serde(rename = "forks_count")]
    pub forks_count: i64,
    #[serde(rename = "mirror_url")]
    pub mirror_url: Value,
    pub archived: bool,
    pub disabled: bool,
    #[serde(rename = "open_issues_count")]
    pub open_issues_count: i64,
    pub license: Value,
    #[serde(rename = "allow_forking")]
    pub allow_forking: bool,
    #[serde(rename = "is_template")]
    pub is_template: bool,
    pub topics: Vec<Value>,
    pub visibility: String,
    pub forks: i64,
    #[serde(rename = "open_issues")]
    pub open_issues: i64,
    pub watchers: i64,
    #[serde(rename = "default_branch")]
    pub default_branch: String,
    pub stargazers: i64,
    #[serde(rename = "master_branch")]
    pub master_branch: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    pub name: String,
    pub email: String,
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
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pusher {
    pub name: String,
    pub email: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sender {
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
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Commit {
    pub id: String,
    #[serde(rename = "tree_id")]
    pub tree_id: String,
    pub distinct: bool,
    pub message: String,
    pub timestamp: String,
    pub url: String,
    pub author: Author,
    pub committer: Committer,
    pub added: Vec<Value>,
    pub removed: Vec<Value>,
    pub modified: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub name: String,
    pub email: String,
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Committer {
    pub name: String,
    pub email: String,
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HeadCommit {
    pub id: String,
    #[serde(rename = "tree_id")]
    pub tree_id: String,
    pub distinct: bool,
    pub message: String,
    pub timestamp: String,
    pub url: String,
    pub author: Author2,
    pub committer: Committer2,
    pub added: Vec<Value>,
    pub removed: Vec<Value>,
    pub modified: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author2 {
    pub name: String,
    pub email: String,
    pub username: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Committer2 {
    pub name: String,
    pub email: String,
    pub username: String,
}
