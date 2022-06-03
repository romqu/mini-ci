use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct GithubWebhookRepository {
    api_client: Client,
}

impl GithubWebhookRepository {
    pub fn new(api_client: Client) -> GithubWebhookRepository {
        GithubWebhookRepository { api_client }
    }

    pub async fn create_webhook(
        &self,
        owner: &'static str,
        repo: &'static str,
        dto: GithubWebhookCreateDto,
    ) -> Result<GithubWebhookDto, reqwest::Error> {
        let url = format!(
            "https://api.github.com/repos/{owner}/{repo}/hooks",
            owner = owner,
            repo = repo
        );

        let response = self
            .api_client
            .post(url)
            .body(serde_json::to_string(&dto))
            .send()
            .await?;

        let github_webhook_dto = response.json::<GithubWebhookDto>().await?;

        Ok(github_webhook_dto)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubWebhookCreateDto {
    pub name: String,
    pub active: bool,
    pub events: Vec<String>,
    pub config: ConfigDto,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubWebhookDto {
    #[serde(rename = "type")]
    pub type_field: String,
    pub id: i64,
    pub name: String,
    pub active: bool,
    pub events: Vec<String>,
    pub config: ConfigDto,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    pub url: String,
    #[serde(rename = "test_url")]
    pub test_url: String,
    #[serde(rename = "ping_url")]
    pub ping_url: String,
    #[serde(rename = "deliveries_url")]
    pub deliveries_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigDto {
    #[serde(rename = "content_type")]
    pub content_type: String,
    #[serde(rename = "insecure_ssl")]
    pub insecure_ssl: String,
    pub url: String,
}
