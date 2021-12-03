use crate::GithubPushEventDto;

pub trait DeployService {
    fn get_key(&self) -> &'static str;
    fn get_repo_url(&self) -> &'static str;
    fn execute(&self, dto: GithubPushEventDto);
}
