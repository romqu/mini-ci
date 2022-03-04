use futures::FutureExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct GithubRepoRepository {
    api_client: Client,
}

impl GithubRepoRepository {
    pub fn new(api_client: Client) -> GithubRepoRepository {
        GithubRepoRepository { api_client }
    }

    pub async fn get_repos(&self, page: u32, per_page: u32) -> reqwest::Result<Vec<GithubRepoDto>> {
        let url = format!(
            "https://api.github.com/user/repos?per_page={}&page={}",
            per_page, page
        );

        let response_result = self.api_client.get(url).send().await;

        let dto_result_future =
            response_result.map(|response| response.json::<Vec<GithubRepoDto>>());

        match dto_result_future {
            Ok(to_json) => to_json.boxed(),
            Err(err) => async { Err(err) }.boxed(),
        }
            .await

        /*        self.api_client.get("").send().and_then(|response| {
                    let json_result = response.json::<Vec<GithubRepoDto>>();

                    json_result.map(move |result: reqwest::Result<Vec<GithubRepoDto>>| {
                        result.map(move |dto_list| {
                            DtoWithHeaders {
                                dto: dto_list,
                                headers: response.headers().clone(),
                            }
                        })
                    })
                })
        */
    }
}

/*pub struct DtoWithHeaders< T> {
    dto: T,
    headers: HeaderMap,
}*/

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubRepoDto {
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    pub name: String,
    #[serde(rename = "full_name")]
    pub full_name: String,
    pub owner: Owner,
    pub private: bool,
    #[serde(rename = "html_url")]
    pub html_url: String,
    pub description: String,
    pub fork: bool,
    pub url: String,
    #[serde(rename = "archive_url")]
    pub archive_url: String,
    #[serde(rename = "assignees_url")]
    pub assignees_url: String,
    #[serde(rename = "blobs_url")]
    pub blobs_url: String,
    #[serde(rename = "branches_url")]
    pub branches_url: String,
    #[serde(rename = "collaborators_url")]
    pub collaborators_url: String,
    #[serde(rename = "comments_url")]
    pub comments_url: String,
    #[serde(rename = "commits_url")]
    pub commits_url: String,
    #[serde(rename = "compare_url")]
    pub compare_url: String,
    #[serde(rename = "contents_url")]
    pub contents_url: String,
    #[serde(rename = "contributors_url")]
    pub contributors_url: String,
    #[serde(rename = "deployments_url")]
    pub deployments_url: String,
    #[serde(rename = "downloads_url")]
    pub downloads_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "forks_url")]
    pub forks_url: String,
    #[serde(rename = "git_commits_url")]
    pub git_commits_url: String,
    #[serde(rename = "git_refs_url")]
    pub git_refs_url: String,
    #[serde(rename = "git_tags_url")]
    pub git_tags_url: String,
    #[serde(rename = "git_url")]
    pub git_url: String,
    #[serde(rename = "issue_comment_url")]
    pub issue_comment_url: String,
    #[serde(rename = "issue_events_url")]
    pub issue_events_url: String,
    #[serde(rename = "issues_url")]
    pub issues_url: String,
    #[serde(rename = "keys_url")]
    pub keys_url: String,
    #[serde(rename = "labels_url")]
    pub labels_url: String,
    #[serde(rename = "languages_url")]
    pub languages_url: String,
    #[serde(rename = "merges_url")]
    pub merges_url: String,
    #[serde(rename = "milestones_url")]
    pub milestones_url: String,
    #[serde(rename = "notifications_url")]
    pub notifications_url: String,
    #[serde(rename = "pulls_url")]
    pub pulls_url: String,
    #[serde(rename = "releases_url")]
    pub releases_url: String,
    #[serde(rename = "ssh_url")]
    pub ssh_url: String,
    #[serde(rename = "stargazers_url")]
    pub stargazers_url: String,
    #[serde(rename = "statuses_url")]
    pub statuses_url: String,
    #[serde(rename = "subscribers_url")]
    pub subscribers_url: String,
    #[serde(rename = "subscription_url")]
    pub subscription_url: String,
    #[serde(rename = "tags_url")]
    pub tags_url: String,
    #[serde(rename = "teams_url")]
    pub teams_url: String,
    #[serde(rename = "trees_url")]
    pub trees_url: String,
    #[serde(rename = "clone_url")]
    pub clone_url: String,
    #[serde(rename = "mirror_url")]
    pub mirror_url: String,
    #[serde(rename = "hooks_url")]
    pub hooks_url: String,
    #[serde(rename = "svn_url")]
    pub svn_url: String,
    pub homepage: String,
    pub language: Value,
    #[serde(rename = "forks_count")]
    pub forks_count: i64,
    #[serde(rename = "stargazers_count")]
    pub stargazers_count: i64,
    #[serde(rename = "watchers_count")]
    pub watchers_count: i64,
    pub size: i64,
    #[serde(rename = "default_branch")]
    pub default_branch: String,
    #[serde(rename = "open_issues_count")]
    pub open_issues_count: i64,
    #[serde(rename = "is_template")]
    pub is_template: bool,
    pub topics: Vec<String>,
    #[serde(rename = "has_issues")]
    pub has_issues: bool,
    #[serde(rename = "has_projects")]
    pub has_projects: bool,
    #[serde(rename = "has_wiki")]
    pub has_wiki: bool,
    #[serde(rename = "has_pages")]
    pub has_pages: bool,
    #[serde(rename = "has_downloads")]
    pub has_downloads: bool,
    pub archived: bool,
    pub disabled: bool,
    pub visibility: String,
    #[serde(rename = "pushed_at")]
    pub pushed_at: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    pub permissions: Permissions,
    #[serde(rename = "allow_rebase_merge")]
    pub allow_rebase_merge: bool,
    #[serde(rename = "template_repository")]
    pub template_repository: Value,
    #[serde(rename = "temp_clone_token")]
    pub temp_clone_token: String,
    #[serde(rename = "allow_squash_merge")]
    pub allow_squash_merge: bool,
    #[serde(rename = "allow_auto_merge")]
    pub allow_auto_merge: bool,
    #[serde(rename = "delete_branch_on_merge")]
    pub delete_branch_on_merge: bool,
    #[serde(rename = "allow_merge_commit")]
    pub allow_merge_commit: bool,
    #[serde(rename = "subscribers_count")]
    pub subscribers_count: i64,
    #[serde(rename = "network_count")]
    pub network_count: i64,
    pub license: License,
    pub forks: i64,
    #[serde(rename = "open_issues")]
    pub open_issues: i64,
    pub watchers: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
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
pub struct Permissions {
    pub admin: bool,
    pub push: bool,
    pub pull: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct License {
    pub key: String,
    pub name: String,
    pub url: String,
    #[serde(rename = "spdx_id")]
    pub spdx_id: String,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
}
