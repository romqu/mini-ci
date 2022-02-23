#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::{env, thread};
use std::any::Any;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::web::Json;
use clap::Parser;
use cmd_lib::{FunChildren, run_cmd, spawn_with_output};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::data::deploy_info_repository::DeployInfoRepository;
use crate::domain::clone_repo_task::CloneRepoTask;
use crate::domain::deploy_service::DeployService;
use crate::domain::init_service::InitService;
use crate::entrypoint::github_push_event_dto::GithubPushEventDto;
use crate::entrypoint::post_github_push_event_handler::handle_post_github_push_event;

mod data;
mod domain;
mod entrypoint;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    cmd_lib::set_pipefail(false);

    let args: Args = Args::parse();
    let git_repo_info_repo = Rc::new(RefCell::new(DeployInfoRepository::new(HashMap::new())));
    let clone_repo_task = CloneRepoTask::new();
    let mut init_service = InitService::new(
        DeployInfoRepository::new(HashMap::new()),
        clone_repo_task,
        args,
    );
    let deploy_service = DeployService::new(git_repo_info_repo.clone());

    init_service.execute();

    let dto = GithubPushEventDto::default();
    let dto1 = GithubPushEventDto {
        ref_field: "refs/heads/mvp".to_string(),
        ..dto
    };


    HttpServer::new(move || {
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
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(long)]
    ssh_passphrase: String,

    #[clap(long)]
    ssh_key_path: String,
}
