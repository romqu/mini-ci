use std::sync::{Arc, Mutex};

use actix_service::Service;
use cmd_lib::FunChildren;
use futures::{FutureExt, StreamExt, TryFutureExt};
use git2::Repository;

use crate::data::deploy_info_repository::{DeployInfoEntity, DeployInfoRepository};
use crate::data::github_repo_repository::{DtoWithHeaders, GithubRepoDto};
use crate::di::start_up_args::StartupArgs;
use crate::domain::clone_repo_task::{CloneRepoTask, CloneRepoTaskError, CloneRepoTaskResult};
use crate::domain::init_service::InitServiceError::{
    CouldNotConvertLinkHeaderValue, CouldNotGetRepos, NoReposFound,
};
use crate::GithubRepoRepository;
use crate::header::HeaderMap;

static REPOS_PER_PAGE: u32 = 100;
static MINI_DOCKER_FILENAME: &str = "mini-docker.yaml";
static GITHUB_CLONE_PATH: &str = "/tmp";

pub struct InitService {
    pub github_repo_repository: GithubRepoRepository,
    pub deploy_info_repo: Arc<Mutex<DeployInfoRepository>>,
    pub clone_repo_task: CloneRepoTask,
    pub args: StartupArgs,
}

impl InitService {
    pub fn new(
        github_repo_repository: GithubRepoRepository,
        deploy_info_repo: Arc<Mutex<DeployInfoRepository>>,
        clone_repo_task: CloneRepoTask,
        args: StartupArgs,
    ) -> InitService {
        InitService {
            github_repo_repository,
            deploy_info_repo,
            clone_repo_task,
            args,
        }
    }

    pub async fn execute(&mut self) -> Result<(), InitServiceError> {
        let github_repos = self.get_all_repos_for_user().await?;
        let sanitized_github_repos = self.remove_archived_and_disabled_repos(github_repos);

        if sanitized_github_repos.is_empty() {
            return Err(NoReposFound);
        }
        let github_repos_with_deploy_file = self
            .filter_repos_by_deploy_file(sanitized_github_repos)
            .await;

        let a = self.clone_repos(github_repos_with_deploy_file)?;

        Ok(())
    }

    async fn get_all_repos_for_user(&self) -> Result<Vec<GithubRepoDto>, InitServiceError> {
        let mut repos: Vec<GithubRepoDto> = vec![];
        let mut page: u32 = 1;

        loop {
            let next_repos = self.get_repos(page, REPOS_PER_PAGE).await?;
            let next_headers = next_repos.headers;
            repos.extend(next_repos.dto);

            if !self.contains_next_link_header(&next_headers)? {
                break;
            }

            page = page + 1;
        }

        if !repos.is_empty() {
            Ok(repos)
        } else {
            Err(NoReposFound)
        }
    }

    async fn get_repos(
        &self,
        page: u32,
        per_page: u32,
    ) -> Result<DtoWithHeaders<Vec<GithubRepoDto>>, InitServiceError> {
        self.github_repo_repository
            .get_user_repos(page, per_page, "owner", "created", "asc")
            .await
            .map_err(|_| CouldNotGetRepos)
    }

    fn contains_next_link_header(&self, headers: &HeaderMap) -> Result<bool, InitServiceError> {
        match headers.get("link") {
            None => Ok(false),
            Some(link_header_value) => {
                link_header_value
                    .to_str()
                    .map_err(|_| CouldNotConvertLinkHeaderValue)
                    .map(|link_header| link_header.contains("next"))
            }
        }
    }

    fn remove_archived_and_disabled_repos(&self, repos: Vec<GithubRepoDto>) -> Vec<GithubRepoDto> {
        repos
            .into_iter()
            .filter(|repo| !repo.archived || !repo.disabled)
            .collect()
    }

    // TODO: parallel (?)
    async fn filter_repos_by_deploy_file(&self, repos: Vec<GithubRepoDto>) -> Vec<GithubRepoDto> {
        let github_user_name = repos.first().unwrap().to_owned().owner.login; // should never fail
        let mut filtered_repos: Vec<GithubRepoDto> = vec![];

        for repo in repos {
            let repo_name = &repo.name;
            let default_branch = &repo.default_branch;
            let url = format!(
                "https://raw.githubusercontent.com/{user}/{repo_name}/{default_branch}/{file_name}",
                user = &github_user_name,
                repo_name = &repo_name,
                default_branch = &default_branch,
                file_name = MINI_DOCKER_FILENAME,
            );

            match self.github_repo_repository.get_headers(url.as_str()).await {
                Ok(response) => {
                    if response.status().is_success() {
                        filtered_repos.push(repo)
                    }
                }
                Err(_error) => {}
            };
        }

        filtered_repos

        /*let a = stream::iter(repos.iter()).filter(|repo| async move {
            let repo_name = &repo.name;
            let default_branch = &repo.default_branch;
            let url = format!(
                "https://raw.githubusercontent.com/{user}/{repo_name}/{default_branch}/{file_name}",
                user = &github_user_name,
                repo_name = &repo_name,
                default_branch = &default_branch,
                file_name = MINI_DOCKER_FILENAME,
            );

            match self.github_repo_repository.get_headers(url.as_str()).await {
                Ok(response) => {
                    true
                },
                Err(_error) => false
            }
        });*/
    }

    /*    fn get_deploy_infos() -> Vec<DeployInfo> {
        /* let contents = fs::read_to_string("deploy-schimmelhof.yml")
            .map_err(|_| CouldNotReadYamlFile)DE

            .and_then(|yaml_text| {
                serde_yaml::from_str::<DeployInfo>(&yaml_text).map_err(|_| CouldNotReadYamlFile)
            });

        spawn_with_output!(bash -c "docker ps");*/

        vec![DeployInfo {
            ssh_git_url: "git@github.com:romqu/schimmelhof-api.git",
            command_builders: vec![
                |path: String| spawn_with_output!(docker-compose -f ${path}/docker-compose.yml build --build-arg ENVPROFILE=dev),
                |path: String| spawn_with_output!(docker-compose -f ${path}/docker-compose.yml up --force-recreate --no-deps -d api),
            ],
        }]
    }*/

    // TODO: parallel (?)
    fn clone_repos(&self, repos: Vec<GithubRepoDto>) -> Result<Vec<TempDataHolderOne>, InitServiceError> {
        repos
            .into_iter()
            .map(|repo| {
                self.clone_repo(repo.ssh_url.clone())
                    .map(|task_result| {
                        TempDataHolderOne {
                            repo_path: task_result.repo_path,
                            git_repository: task_result.git_repository,
                            github_repo: repo,
                        }
                    })
                    .map_err(|_| InitServiceError::CouldNotCloneRepo)
            })
            .collect()
    }

    fn clone_repo(
        &self,
        ssh_git_url: String,
    ) -> Result<CloneRepoTaskResult, CloneRepoTaskError> {
        self.clone_repo_task.execute(
            ssh_git_url,
            GITHUB_CLONE_PATH,
            &self.args.ssh_passphrase,
            &self.args.ssh_key_path,
        )
    }

    fn parse_deploy_commands(&self, repos: Vec<GithubRepoDto>) {}

    fn save_deploy_infos(
        &mut self,
        data_vec: Vec<TempDataHolderOne>,
    ) -> Result<(), InitServiceError> {
        let deploy_infos = data_vec.into_iter().map(|data| {
            DeployInfoEntity {
                ssh_git_url: data.ssh_git_url,
                command_builders: data.command_builders,
                repo_path: data.repo_path,
                git_repository: data.git_repository,
            }
        });

        for deploy_info in deploy_infos {
            self.deploy_info_repo
                .lock()
                .unwrap()
                .save(deploy_info.ssh_git_url.to_string(), deploy_info);
        }

        Ok({})
    }
}

pub struct TempDataHolderOne {
    pub github_repo: GithubRepoDto,
    pub repo_path: String,
    pub git_repository: Repository,
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

#[derive(Debug)]
pub enum InitServiceError {
    CouldNotGetRepos,
    NoReposFound,
    CouldNotReadYamlFile,
    CouldNotParseYamlFile,
    CouldNotCloneRepo,
    CouldNotConvertLinkHeaderValue,
}
