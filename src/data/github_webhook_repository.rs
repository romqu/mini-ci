use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::data::api_call_delegate::{ApiCallDelegate, ApiCallError};

pub struct GithubWebhookRepository {
    api_delegate: Arc<Mutex<ApiCallDelegate>>,
}

impl GithubWebhookRepository {
    pub fn new(api_delegate: Arc<Mutex<ApiCallDelegate>>) -> GithubWebhookRepository {
        GithubWebhookRepository { api_delegate }
    }

    pub async fn create_webhook(
        &self,
        owner: &'static str,
        repo: &'static str,
        dto: GithubWebhookCreateDto,
    ) -> Result<Box<GithubWebhookCreateDto>, ApiCallError> {
        let url = format!(
            "https://api.github.com/repos/{owner}/{repo}/hooks",
            owner = owner,
            repo = repo
        );

        self.api_delegate.lock().unwrap().execute_post_call(url, &dto).await
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
