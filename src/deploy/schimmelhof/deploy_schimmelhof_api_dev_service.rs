use crate::deploy::deploy_service::DeployService;
use crate::{GithubPushEventDto, RepoInfoRepository};
use git2::build::CheckoutBuilder;
use git2::{BranchType, ObjectType};
use std::cell::RefCell;
use std::rc::Rc;

pub struct DeploySchimmelhofApiDevService {
    repo_info_repo: Rc<RefCell<RepoInfoRepository>>,
}

impl DeploySchimmelhofApiDevService {
    pub fn new(repo_info_repo: Rc<RefCell<RepoInfoRepository>>) -> DeploySchimmelhofApiDevService {
        return DeploySchimmelhofApiDevService { repo_info_repo };
    }
}

impl DeployService for DeploySchimmelhofApiDevService {
    fn ssh_git_url(&self) -> &'static str {
        return "git@github.com:romqu/schimmelhof-api.git";
    }

    fn execute(&self, dto: GithubPushEventDto) {
        self.repo_info_repo
            .borrow()
            .get(self.ssh_git_url())
            .map(|repo_info| {

                dto.ref_field.rfind("/").unwrap();

                let branch = repo_info
                    .repository
                    .find_branch("origin/mvp", BranchType::Remote)
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

                repo_info.repository.set_head("refs/heads/mvp").unwrap()
            });
    }
}
