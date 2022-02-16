use std::cell::RefCell;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use std::thread;
use std::thread::JoinHandle;

use cmd_lib::*;
use git2::{Branch, BranchType, ObjectType};
use git2::build::CheckoutBuilder;

use crate::{GithubPushEventDto, GitRepoInfoRepository};
use crate::data::repo_info_repository::GitRepoInfoEntity;
use crate::deploy::deploy_service::DeployService;
use crate::deploy::schimmelhof::deploy_schimmelhof_api_dev_service::DeploySchimmelhofApiDevServiceError::{CouldNotCheckoutBranch, CouldNotGetBranch, CouldNotGetRepoInfo};

pub struct DeploySchimmelhofApiDevService {
    git_repo_info_repo: Rc<RefCell<GitRepoInfoRepository>>,
}

impl DeploySchimmelhofApiDevService {
    pub fn new(
        repo_info_repo: Rc<RefCell<GitRepoInfoRepository>>,
    ) -> DeploySchimmelhofApiDevService {
        return DeploySchimmelhofApiDevService {
            git_repo_info_repo: repo_info_repo,
        };
    }
}

impl DeployService<String, DeploySchimmelhofApiDevServiceError> for DeploySchimmelhofApiDevService {
    fn ssh_git_url(&self) -> &'static str {
        return "git@github.com:romqu/schimmelhof-api.git";
    }

    fn execute(
        &self,
        dto: GithubPushEventDto,
    ) -> Result<String, DeploySchimmelhofApiDevServiceError> {
        return self
            .git_repo_info_repo
            .borrow()
            .get(self.ssh_git_url())
            .ok_or(CouldNotGetRepoInfo)
            .and_then(|git_repo_info| Self::get_branch(dto, git_repo_info))
            .and_then(|temp_data_holder| Self::checkout_branch(temp_data_holder))
            .map(|repo_info| Self::execute_deploy_commands(repo_info))
            .map(|_| "".to_string());
    }
}

struct TempDataHolderOne<'a> {
    branch: Branch<'a>,
    git_repo_info: &'a GitRepoInfoEntity,
    refs: String,
}

impl DeploySchimmelhofApiDevService {
    fn get_branch(
        dto: GithubPushEventDto,
        git_repo_info: &GitRepoInfoEntity,
    ) -> Result<TempDataHolderOne, DeploySchimmelhofApiDevServiceError> {
        let refs = dto.ref_field;

        refs.rfind("/")
            .ok_or(CouldNotGetBranch)
            .and_then(|position| {
                let branch_name = &refs[position..];
                let formatted_branch_name = format!("origin/{}", branch_name);

                git_repo_info
                    .git_repository
                    .find_branch(formatted_branch_name.as_str(), BranchType::Remote)
                    .map(|branch| {
                        TempDataHolderOne {
                            branch,
                            git_repo_info,
                            refs,
                        }
                    })
                    .map_err(|_| CouldNotGetBranch)
            })
    }

    fn checkout_branch(
        first: TempDataHolderOne,
    ) -> Result<&GitRepoInfoEntity, DeploySchimmelhofApiDevServiceError> {
        let git_repo_info = first.git_repo_info;
        let git_repository = &git_repo_info.git_repository;
        let branch = first.branch;
        let refs = first.refs;

        branch
            .get()
            .peel_to_tree()
            .and_then(|tree| git_repo_info.git_repository.find_tree(tree.id()))
            .and_then(|tree| git_repository.find_object(tree.id(), Option::Some(ObjectType::Tree)))
            .and_then(|git_object| {
                git_repository.checkout_tree(&git_object, Some(CheckoutBuilder::default().force()))
            })
            .and_then(|_| git_repository.set_head(refs.as_str()))
            .map_err(|_| CouldNotCheckoutBranch)
            .map(|_| git_repo_info)
    }

    fn execute_deploy_commands(repo_info: &GitRepoInfoEntity) -> JoinHandle<()> {
        let path = &repo_info.path;
        let path_copy = path.clone();

        thread::spawn(move || {
            let commands = vec![
                spawn_with_output!(docker-compose -f ${path_copy}/docker-compose.yml build --build-arg ENVPROFILE=dev),
                spawn_with_output!(docker-compose -f ${path_copy}/docker-compose.yml up --force-recreate --no-deps -d api),
            ];

            for command in commands {
                command.unwrap().wait_with_pipe(&mut |pipe| {
                    BufReader::new(pipe)
                        .lines()
                        .filter_map(|line| line.ok())
                        .for_each(|line| println!("{}", line));
                });
            }
        })
    }
}

#[derive(Debug)]
pub enum DeploySchimmelhofApiDevServiceError {
    CouldNotGetBranch,
    CouldNotGetRepoInfo,
    CouldNotCheckoutBranch,
}
