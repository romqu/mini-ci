#![feature(once_cell)]
#![feature(map_try_insert)]
extern crate lazy_static;
extern crate regex;

use actix_web::{App, HttpServer, web};
use curl::easy::Easy;
use serde::{Deserialize, Serialize};

use untitled::entrypoint::post_github_push_event_handler::handle_post_github_push_event;
use untitled::InitError;

#[actix_web::main]
async fn main() -> Result<(), InitError> {
    let mut easy = Easy::new();
    easy.url("https://raw.githubusercontent.com/romqu/ubuntu-config-scripts/master/README.m?token=GHSAT0AAAAAABR6TMBHGSVKHW4HAVLNYJIYQZGTZA").unwrap();
    easy.perform().unwrap();

    let a = easy.response_code().unwrap();

    Ok({})

    /*    match init_app() {
            Ok(_) => start_app().await.map_err(|_| CouldNotStartApp),
            Err(err) => Err(err),
        }
    */
}

pub async fn start_app() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route(
            "/api/v1/events/push",
            web::post().to(handle_post_github_push_event),
        )
    })
        .bind("0.0.0.0:8083")?
        .run()
        .await
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeployInfo {
    pub url: String,
    pub branches: Vec<Branch>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Branch {
    pub name: String,
    pub commands: Vec<String>,
}