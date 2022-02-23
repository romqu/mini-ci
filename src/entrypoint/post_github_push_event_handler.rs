use std::thread::JoinHandle;

use actix_web::{HttpResponse, web};
use actix_web::web::Json;

use crate::{DEPLOY_SERVICE_CELL, DeployService, GithubPushEventDto};

pub async fn handle_post_github_push_event(
    json: Json<GithubPushEventDto>,
) -> HttpResponse {
    match DEPLOY_SERVICE_CELL.get().unwrap().execute(json.into_inner()) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}
