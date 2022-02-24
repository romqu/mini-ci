#![feature(once_cell)]
#![feature(map_try_insert)]
extern crate lazy_static;
extern crate regex;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use actix_web::{App, HttpServer, web};
use clap::Parser;

use untitled::data::deploy_info_repository::DeployInfoRepository;
use untitled::di::singletons::DEPLOY_SERVICE_CELL;
use untitled::di::start_up_args::StartupArgs;
use untitled::domain::clone_repo_task::CloneRepoTask;
use untitled::domain::deploy_service::DeployService;
use untitled::domain::init_service::InitService;
use untitled::entrypoint::post_github_push_event_handler::handle_post_github_push_event;

use crate::MainError::{CouldNotInitApp, CouldNotInitDependencies, CouldNotStartApp};

#[actix_web::main]
async fn main() -> Result<(), MainError> {
    match init_app() {
        Ok(_) => start_app().await.map_err(|_| CouldNotStartApp),
        Err(err) => Err(err),
    }
}

fn init_app() -> Result<(), MainError> {
    init_dependencies()
        .and_then(|mut init_service| init_service.execute().map_err(|_| CouldNotInitApp))
}

fn init_dependencies() -> Result<InitService, MainError> {
    let args: StartupArgs = StartupArgs::parse();
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

#[derive(Debug)]
enum MainError {
    CouldNotInitDependencies,
    CouldNotInitApp,
    CouldNotStartApp,
}
