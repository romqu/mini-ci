use std::future::Future;

use actix_service::Service;
use futures::FutureExt;
use reqwest::{Client, Error, Response};

pub struct GithubRepoRepository {
    api_client: Client,
}

impl GithubRepoRepository {
    pub fn new(api_client: Client) -> GithubRepoRepository {
        GithubRepoRepository { api_client }
    }

    pub async fn get_repos(&self, page: u32, per_page: u32) {
        let a = self
            .api_client
            .get("")
            .send()
            .map(|a: Result<Response, Error>| a);
    }
}

trait FutureResultExt<O> {
    fn map_result<S, F>(self) -> dyn Future<Output=Result<O, F>>;
}

impl<O, S, F> FutureResultExt<O> for dyn Future<Output=Result<S, F>> {
    fn map_result(self) -> &dyn Future<Output=Result<O, F>> {}
}
