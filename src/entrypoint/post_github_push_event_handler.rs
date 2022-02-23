use std::thread::JoinHandle;

use actix_web::{HttpResponse, web};
use actix_web::web::Json;

use crate::{DeployService, GithubPushEventDto};

pub async fn handle_post_github_push_event(
    data: web::Data<DeployService>,
    json: Json<GithubPushEventDto>,
) -> HttpResponse {
    match data.execute(json.into_inner()) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}
