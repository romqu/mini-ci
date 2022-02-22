use cmd_lib::FunChildren;
use futures::stream::iter;
use git2::Repository;

use crate::{Args, CloneRepoTask, spawn_with_output};
use crate::data::deploy_info_repository::{DeployInfoEntity, DeployInfoRepository};
use crate::domain::clone_repo_task::{CloneRepoTaskError, CloneRepoTaskResult};
use crate::domain::clone_repo_task::CloneRepoTaskError::CouldNotCloneRepo;

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
        let deploy_infos = Self::get_deploy_infos();
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

    pub fn clone_repos(&self, deploy_infos: &Vec<DeployInfo>) {
        let args = &self.args;

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
            });
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
}

pub struct TempDataHolderOne {
    pub ssh_git_url: &'static str,
    pub command_builders: Vec<fn(String) -> std::io::Result<FunChildren>>,
    pub path: String,
    pub git_repository: Repository,
}

pub struct DeployInfo {
    pub ssh_git_url: &'static str,
    pub command_builders: Vec<fn(String) -> std::io::Result<FunChildren>>,
}
