use cmd_lib::FunChildren;
use futures::stream::iter;
use git2::Repository;

use crate::{Args, CloneRepoTask, spawn_with_output};
use crate::data::deploy_info_repository::{DeployInfoEntity, DeployInfoRepository};
use crate::domain::clone_repo_task::{CloneRepoTaskError, CloneRepoTaskResult};
use crate::domain::clone_repo_task::CloneRepoTaskError::CouldNotCloneRepo;
use crate::domain::init_service::InitServiceError::CouldNotSaveDeployInfo;

pub struct InitService {
    pub deploy_info_repo: DeployInfoRepository,
    pub clone_repo_task: CloneRepoTask,
    pub args: Args,
}

impl InitService {
    pub fn new(
        deploy_info_repo: DeployInfoRepository,
        clone_repo_task: CloneRepoTask,
        args: Args,
    ) -> InitService {
        InitService {
            deploy_info_repo,
            clone_repo_task,
            args,
        }
    }

    pub fn execute(&mut self) {
        self.clone_repos(&Self::get_deploy_infos())
            .and_then(|data| self.save_deploy_infos(data));
    }

    fn get_deploy_infos() -> Vec<DeployInfo> {
        vec![DeployInfo {
            ssh_git_url: "git@github.com:romqu/schimmelhof-api.git",
            command_builders: vec![
                |path: String| spawn_with_output!(docker-compose -f ${path}/docker-compose.yml build --build-arg ENVPROFILE=dev),
                |path: String| spawn_with_output!(docker-compose -f ${path}/docker-compose.yml up --force-recreate --no-deps -d api),
            ],
        }]
    }

    pub fn clone_repos(
        &self,
        deploy_infos: &Vec<DeployInfo>,
    ) -> Result<Vec<TempDataHolderOne>, InitServiceError> {
        deploy_infos
            .iter()
            .map(|deploy_info| {
                self.clone_repo(&self.args, deploy_info)
                    .map(|task_result| {
                        TempDataHolderOne {
                            ssh_git_url: deploy_info.ssh_git_url,
                            command_builders: deploy_info.command_builders.clone(),
                            repo_path: task_result.repo_path,
                            git_repository: task_result.git_repository,
                        }
                    })
                    .map_err(|_| InitServiceError::CouldNotCloneRepo)
            })
            .collect()
        /*
        let start: Vec<Result<CloneRepoTaskResult, CloneRepoTaskError>> = vec![];
        deploy_infos
        .iter()
        .fold(start, |mut previous, deploy_info| {
            let result = match previous.last() {
                None => self.clone_repo(args, deploy_info),
                Some(previousResult) => {
                    previousResult.as_ref().map_err(Clone::clone).and_then(|_| self.clone_repo(args, deploy_info))
                }
            };

            previous.push(result);
            previous
        });*/
    }

    fn clone_repo(
        &self,
        args: &Args,
        deploy_info: &DeployInfo,
    ) -> Result<CloneRepoTaskResult, CloneRepoTaskError> {
        self.clone_repo_task.execute(
            deploy_info.ssh_git_url,
            "/tmp",
            &args.ssh_passphrase,
            &args.ssh_key_path,
        )
    }

    fn save_deploy_infos(
        &mut self,
        temp_data_holder_one: Vec<TempDataHolderOne>,
    ) -> Result<Vec<DeployInfoEntity>, InitServiceError> {
        temp_data_holder_one
            .into_iter()
            .map(|data| {
                let entity = DeployInfoEntity {
                    ssh_git_url: data.ssh_git_url,
                    command_builders: data.command_builders,
                    repo_path: data.repo_path,
                    git_repository: data.git_repository,
                };

                self.deploy_info_repo
                    .save(entity.ssh_git_url.to_string(), entity)
                    .ok_or(CouldNotSaveDeployInfo)
            })
            .collect()
    }
}

pub struct TempDataHolderOne {
    pub ssh_git_url: &'static str,
    pub command_builders: Vec<fn(String) -> std::io::Result<FunChildren>>,
    pub repo_path: String,
    pub git_repository: Repository,
}

pub struct DeployInfo {
    pub ssh_git_url: &'static str,
    pub command_builders: Vec<fn(String) -> std::io::Result<FunChildren>>,
}

enum InitServiceError {
    CouldNotCloneRepo,
    CouldNotSaveDeployInfo,
}
