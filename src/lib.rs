#![feature(once_cell)]
#![feature(map_try_insert)]


use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use clap::Parser;

use crate::data::deploy_info_repository::DeployInfoRepository;
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


pub fn init_app() -> Result<(), InitError> {
    init_dependencies()
        .and_then(|mut init_service| init_service.execute().map_err(|_| CouldNotInitApp))
}

fn init_dependencies() -> Result<InitService, InitError> {
    let args: StartupArgs = StartupArgs::parse();
    let deploy_info_repository = Arc::new(Mutex::new(DeployInfoRepository::new(HashMap::new())));
    let clone_repo_task = CloneRepoTask::new();
    let init_service = InitService::new(
        deploy_info_repository.clone(),
        clone_repo_task,
        args,
    );

    DEPLOY_SERVICE_CELL
        .set(DeployService::new(deploy_info_repository.clone()))
        .map_err(|_| CouldNotInitDependencies)
        .map(|_| init_service)
}

#[derive(Debug)]
pub enum InitError {
    CouldNotInitDependencies,
    CouldNotInitApp,
    CouldNotStartApp,
}