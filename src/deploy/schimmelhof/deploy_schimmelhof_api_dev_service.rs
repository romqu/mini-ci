use crate::deploy::deploy_service::DeployService;
use crate::{GithubPushEventDto, RepoInfoRepository};
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
    fn get_key(&self) -> &'static str {
        return "refs/heads/dev";
    }

    fn get_repo_url(&self) -> &'static str {
        return "git@github.com:romqu/schimmelhof-api.git";
    }

    fn execute(&self, _: GithubPushEventDto) {
        self.repo_info_repo
            .borrow()
            .get(self.get_repo_url())
            .map(|repo_info| {});
    }
}
