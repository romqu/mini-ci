use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use reqwest::{Client, Response};
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

pub struct GithubRepoRepository {
    api_client: Arc<Mutex<Client>>,
}

impl GithubRepoRepository {
    pub fn new(api_client: Arc<Mutex<Client>>) -> GithubRepoRepository {
        GithubRepoRepository { api_client }
    }

    pub async fn get_user_repos(
        &self,
        page: u32,
        per_page: u32,
        owner_type: &'static str,
        sort_by: &'static str,
        sort_direction: &'static str,
    ) -> Result<DtoWithHeaders<Vec<GithubRepoDto>>, reqwest::Error> {
        let url = format!(
            "https://api.github.com/user/repos?per_page={}&page={}&type={}&sort={}&direction={}",
            per_page, page, owner_type, sort_by, sort_direction
        );

        let response = self.api_client.lock().unwrap().get(url).send().await?;

        let headers = response.headers().clone();

        let github_repo_list_dto = response.json::<Vec<GithubRepoDto>>().await?;

        Ok(DtoWithHeaders {
            dto: github_repo_list_dto.clone(),
            headers,
        })

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

    pub async fn get_headers(&self, url: &str) -> Result<Response, reqwest::Error> {
        self.api_client.lock().unwrap().head(url).send().await
    }
}

pub struct DtoWithHeaders<T> {
    pub dto: T,
    pub headers: HeaderMap,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubRepoDto {
    pub id: i64,
    pub name: String,
    #[serde(rename = "full_name")]
    pub full_name: String,
    pub owner: Owner,
    #[serde(rename = "ssh_url")]
    pub ssh_url: String,
    #[serde(rename = "default_branch")]
    pub default_branch: String,
    pub archived: bool,
    pub disabled: bool,
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    pub login: String,
}

#[derive(Clone, Copy)]
enum Error {
    No,
}
