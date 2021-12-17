use std::any::Any;

use crate::GithubPushEventDto;

pub trait DeployService<T, E> {
    fn ssh_git_url(&self) -> &'static str;
    fn execute(&self, dto: GithubPushEventDto) -> Result<T, E>;
}
