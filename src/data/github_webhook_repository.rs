use std::sync::{Arc, Mutex};

use actix_service::Service;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use crate::data::github_webhook_repository::ApiCallError::{DtoToJsonStringError, JsonToDtoError, SendError};

pub struct GithubWebhookRepository {
    api_client: Arc<Mutex<Client>>,
}

impl GithubWebhookRepository {
    pub fn new(api_client: Arc<Mutex<Client>>) -> GithubWebhookRepository {
        GithubWebhookRepository { api_client }
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

        self.execute_post_call(url, &dto).await
    }

    // static string causes hidden lifetime
    async fn execute_post_call<T>(&self, url: String, dto: &T) -> Result<Box<T>, ApiCallError>
        where
            T: ?Sized + Serialize + DeserializeOwned,
    {
        let body = serde_json::to_string(&dto).map_err(|_| DtoToJsonStringError)?;

        self.api_client
            .lock()
            .unwrap()
            .post(url)
            .body(body)
            .send()
            .await
            .map_err(|err| {
                println!("{}", err);
                SendError
            })?
            .json::<Box<T>>()
            .await
            .map_err(|err| {
                println!("{}", err);
                JsonToDtoError
            })
    }
}

pub enum ApiCallError {
    DtoToJsonStringError,
    SendError,
    JsonToDtoError,
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
