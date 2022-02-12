use std::cell::RefCell;
use std::rc::Rc;

use cmd_lib::run_cmd;
use git2::{Branch, BranchType, ObjectType};
use git2::build::CheckoutBuilder;

use crate::{GithubPushEventDto, RepoInfoRepository};
use crate::data::repo_info_repository::RepoInfoEntity;
use crate::deploy::deploy_service::DeployService;
use crate::deploy::schimmelhof::deploy_schimmelhof_api_dev_service::DeploySchimmelhofApiDevServiceError::{CouldNotExecuteScript, CouldNotGetBranch, CouldNotGetRepoInfo, CouldNotSetHead};

pub struct DeploySchimmelhofApiDevService {
    repo_info_repo: Rc<RefCell<RepoInfoRepository>>,
}

impl DeploySchimmelhofApiDevService {
    pub fn new(repo_info_repo: Rc<RefCell<RepoInfoRepository>>) -> DeploySchimmelhofApiDevService {
        return DeploySchimmelhofApiDevService { repo_info_repo };
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
            .repo_info_repo
            .borrow()
            .get(self.ssh_git_url())
            .ok_or(CouldNotGetRepoInfo)
            .and_then(|repo_info| {
                let refs = dto.ref_field;
                let position = refs.rfind("/").unwrap();
                let branch_name = &refs[position..];
                let formatted_branch_name = format!("origin/{}", branch_name);

                let branch = repo_info
                    .repository
                    .find_branch(formatted_branch_name.as_str(), BranchType::Remote)
                    .unwrap();

                let tree = repo_info
                    .repository
                    .find_tree(branch.get().peel_to_tree().unwrap().id())
                    .unwrap();

                let git_object = repo_info
                    .repository
                    .find_object(tree.id(), Option::Some(ObjectType::Tree))
                    .unwrap();

                repo_info
                    .repository
                    .checkout_tree(&git_object, Some(CheckoutBuilder::default().force()))
                    .unwrap();

                repo_info
                    .repository
                    .set_head(refs.as_str())
                    .map_err(|_| CouldNotSetHead)
                    .map(|_| repo_info)
            })
            .and_then(|repo_info| self.execute_deploy_script(repo_info))
            .map(|_| "".to_string());
    }
}

impl DeploySchimmelhofApiDevService {
    fn execute_deploy_script(
        &self,
        repo_info: &RepoInfoEntity,
    ) -> Result<(), DeploySchimmelhofApiDevServiceError> {
        let path = &repo_info.path;
        run_cmd!(
            /bin/bash ${path}/deploy.sh -t dev;
        )
        .map_err(|err| {
            println!("error: {}", err);

            CouldNotExecuteScript
        })
    }
}

#[derive(Debug)]
pub enum DeploySchimmelhofApiDevServiceError {
    CouldNotGetBranch,
    CouldNotGetRepoInfo,
    CouldNotSetHead,
    CouldNotExecuteScript,
}
