#![feature(once_cell)]
#![feature(map_try_insert)]

use std::collections::HashMap;
use std::fmt::Display;
use std::sync::{Arc, Mutex};

use actix_service::Service;
use clap::Parser;
use reqwest::{Client, header};
use reqwest::header::HeaderValue;

use crate::data::deploy_info_repository::DeployInfoRepository;
use crate::data::github_repo_repository::GithubRepoRepository;
use crate::data::github_webhook_repository::GithubWebhookRepository;
use crate::di::singletons::DEPLOY_SERVICE_CELL;
use crate::di::start_up_args::StartupArgs;
use crate::domain::clone_repo_task::CloneRepoTask;
use crate::domain::deploy_service::DeployService;
use crate::domain::init_service::InitService;
use crate::InitError::{CouldNotInitApp, CouldNotInitDependencies};

pub mod data;
pub mod di;
pub mod domain;
pub mod entrypoint;

pub async fn init_app() -> Result<(), InitError> {
    let mut init_service = init_dependencies()?;

    init_service.execute().await.map_err(|_| CouldNotInitApp)
}

fn init_dependencies() -> Result<InitService, InitError> {
    let args: StartupArgs = StartupArgs::parse();
    let github_token = env!("GITHUB_TOKEN");

    init_github_api_client(github_token.to_string()).and_then(|api_client| {
        let deploy_info_repository =
            Arc::new(Mutex::new(DeployInfoRepository::new(HashMap::new())));
        let a = Arc::new(Mutex::new(api_client));
        let github_repo_repository = GithubRepoRepository::new(a.clone());
        let github_repo_repository = GithubWebhookRepository::new(api_client);
        let clone_repo_task = CloneRepoTask::new();
        let init_service = InitService::new(
            github_repo_repository,
            deploy_info_repository.clone(),
            clone_repo_task,
            args,
        );

        DEPLOY_SERVICE_CELL
            .set(DeployService::new(deploy_info_repository.clone()))
            .map_err(|_| CouldNotInitDependencies)
            .map(|_| init_service)
    })
}

fn init_github_api_client(github_token: String) -> Result<Client, InitError> {
    HeaderValue::from_str(("Bearer ".to_owned().clone() + github_token.as_str()).as_str())
        .map_err(|_| CouldNotInitDependencies)
        .and_then(|bearer_header| {
            let mut default_headers = header::HeaderMap::new();

            default_headers.insert(header::USER_AGENT, HeaderValue::from_static("reqwest"));
            default_headers.insert(
                header::ACCEPT,
                HeaderValue::from_static("application/vnd.github.v3+json"),
            );
            default_headers.insert(header::AUTHORIZATION, bearer_header);

            Client::builder()
                .default_headers(default_headers)
                .build()
                .map_err(|_| CouldNotInitDependencies)
        })
}

#[derive(Debug)]
pub enum InitError {
    CouldNotInitDependencies,
    CouldNotInitApp,
    CouldNotStartApp,
}
