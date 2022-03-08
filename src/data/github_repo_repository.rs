use chrono::{DateTime, Utc};
use reqwest::Client;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

pub struct GithubRepoRepository {
    api_client: Client,
}

impl GithubRepoRepository {
    pub fn new(api_client: Client) -> GithubRepoRepository {
        GithubRepoRepository { api_client }
    }

    pub async fn get_repos(
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

        let response = self.api_client.get(url).send().await?;

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
    #[serde(rename = "ssh_url")]
    pub ssh_url: String,
    #[serde(rename = "default_branch")]
    pub default_branch: String,
    pub archived: bool,
    pub disabled: bool,
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Copy)]
enum Error {
    No,
}
