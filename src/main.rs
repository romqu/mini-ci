#![feature(once_cell)]
#![feature(map_try_insert)]
#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::lazy::SyncOnceCell;
use std::sync::{Arc, Mutex};

use actix_web::{App, HttpServer, web};
use clap::Parser;
use cmd_lib::spawn_with_output;

use crate::data::deploy_info_repository::DeployInfoRepository;
use crate::domain::clone_repo_task::CloneRepoTask;
use crate::domain::deploy_service::DeployService;
use crate::domain::init_service::InitService;
use crate::entrypoint::github_push_event_dto::GithubPushEventDto;
use crate::entrypoint::post_github_push_event_handler::handle_post_github_push_event;
use crate::MainError::{CouldNotInitApp, CouldNotInitDependencies, CouldNotStartApp};

mod data;
mod domain;
mod entrypoint;

static DEPLOY_SERVICE_CELL: SyncOnceCell<DeployService> = SyncOnceCell::new();

#[actix_web::main]
async fn main() -> Result<(), MainError> {
    match init_app() {
        Ok(_) => start_app().await.map_err(|_| CouldNotStartApp),
        Err(err) => Err(err),
    }

    /*    let dto = GithubPushEventDto::default();
    let dto1 = GithubPushEventDto {
        ref_field: "refs/heads/mvp".to_string(),
        ..dto
    };*/
}

fn init_app() -> Result<(), MainError> {
    init_dependencies()
        .and_then(|mut init_service| init_service.execute().map_err(|err| {
            println!("{:?}", err);
            CouldNotInitApp
        }))
}

fn init_dependencies() -> Result<InitService, MainError> {
    let args: Args = Args::parse();
    let git_repo_info_repo = Arc::new(Mutex::new(DeployInfoRepository::new(HashMap::new())));
    let clone_repo_task = CloneRepoTask::new();
    let init_service = InitService::new(
        DeployInfoRepository::new(HashMap::new()),
        clone_repo_task,
        args,
    );

    DEPLOY_SERVICE_CELL
        .set(DeployService::new(git_repo_info_repo.clone()))
        .map_err(|_| CouldNotInitDependencies)
        .map(|_| init_service)
}

async fn start_app() -> std::io::Result<()> {
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

#[derive(Parser, Debug)]
#[clap(long_about = None)]
pub struct Args {
    #[clap(long)]
    ssh_passphrase: String,

    #[clap(long)]
    ssh_key_path: String,
}

#[derive(Debug)]
enum MainError {
    CouldNotInitDependencies,
    CouldNotInitApp,
    CouldNotStartApp,
}
