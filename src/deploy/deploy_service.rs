use crate::GithubPushEventDto;

pub trait DeployService {
    fn ssh_git_url(&self) -> &'static str;
    fn execute(&self, dto: GithubPushEventDto);
}
