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
        self.save_deploy_infos()
    }

    pub fn clone_repos() {
        let deploy_infos = vec![DeployInfoEntity {
            ssh_git_url: "git@github.com:romqu/schimmelhof-api.git",
            command_builders: vec![
                |path: String| spawn_with_output!(docker-compose -f ${path}/docker-compose.yml build --build-arg ENVPROFILE=dev),
                |path: String| spawn_with_output!(docker-compose -f ${path}/docker-compose.yml up --force-recreate --no-deps -d api),
            ],
        }];

        for deploy_info in deploy_infos {
            clone_repo_task.execute(
                deploy_info.ssh_git_url,
                "/tmp",
                &args.ssh_passphrase,
                &args.ssh_key_path,
            );
        }
    }

    pub fn save_deploy_infos(&mut self) {
        let deploy_infos = vec![DeployInfoEntity {
            ssh_git_url: "git@github.com:romqu/schimmelhof-api.git",
            command_builders: vec![
                |path: String| spawn_with_output!(docker-compose -f ${path}/docker-compose.yml build --build-arg ENVPROFILE=dev),
                |path: String| spawn_with_output!(docker-compose -f ${path}/docker-compose.yml up --force-recreate --no-deps -d api),
            ],
        }];

        for deploy_info in deploy_infos {
            self.deploy_info_repo
                .save(deploy_info.ssh_git_url.to_string(), deploy_info);
        }
    }
}
