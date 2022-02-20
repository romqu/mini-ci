use cmd_lib::FunChildren;

use crate::{Args, CloneRepoTask, spawn_with_output};
use crate::data::deploy_info_repository::{DeployInfoEntity, DeployInfoRepository};

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

    pub fn clone_repos(&self, deploy_infos: Vec<DeployInfo>) {
        let args = &self.args;

        let a = deploy_infos.iter();

        self.clone_repo_task.execute(
            deploy_info.ssh_git_url,
            "/tmp",
            &args.ssh_passphrase,
            &args.ssh_key_path,
        );
    }
}

pub struct DeployInfo {
    pub ssh_git_url: &'static str,
    pub command_builders: Vec<fn(String) -> std::io::Result<FunChildren>>,
}
