use actix_web::HttpResponse;
use actix_web::web::Json;

use crate::di::singletons::DEPLOY_SERVICE_CELL;
use crate::entrypoint::github_push_event_dto::GithubPushEventDto;

pub async fn handle_post_github_push_event(json: Json<GithubPushEventDto>) -> HttpResponse {
    match DEPLOY_SERVICE_CELL
        .get()
        .unwrap()
        .execute(json.into_inner())
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}
