use actix_web::{App, test, web};
use futures::executor::block_on;

use untitled::entrypoint::github_push_event_dto::{GithubPushEventDto, Repository};
use untitled::entrypoint::post_github_push_event_handler::handle_post_github_push_event;
use untitled::init_app;

fn main() {
    block_on(test());
}

async fn test() {
    init_app().await;

    let dto = GithubPushEventDto::default();
    let post_dto = GithubPushEventDto {
        ref_field: "refs/heads/mvp".to_string(),
        repository: Repository {
            ssh_url: "git@github.com:romqu/schimmelhof-api.git".to_string(),
            ..dto.repository
        },
        ..dto
    };

    let path = "/api/v1/events/push";
    let mut app =
        test::init_service(App::new().route(path, web::post().to(handle_post_github_push_event)))
            .await;
    let req = test::TestRequest::post()
        .set_json(&post_dto)
        .uri(path)
        .to_request();
    let resp = test::call_service(&mut app, req).await;

    assert!(resp.status().is_success());
}
