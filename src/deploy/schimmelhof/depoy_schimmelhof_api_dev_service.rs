use crate::deploy::deploy_service::DeployService;
use crate::GithubPushEventDto;

pub struct DeploySchimmelhofApiDevService;

impl DeploySchimmelhofApiDevService {
    pub fn new() -> DeploySchimmelhofApiDevService {
        return DeploySchimmelhofApiDevService;
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
        todo!()
    }
}
