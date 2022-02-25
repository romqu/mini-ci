#![feature(once_cell)]
#![feature(map_try_insert)]
extern crate lazy_static;
extern crate regex;

use std::fs;
use std::io::{BufRead, BufReader};

use actix_web::{App, HttpServer, web};
use cmd_lib::spawn_with_output;
use serde::{Deserialize, Serialize};

use untitled::InitError;
use untitled::entrypoint::post_github_push_event_handler::handle_post_github_push_event;

#[actix_web::main]
async fn main() -> Result<(), InitError> {
    let contents = fs::read_to_string("deploy-schimmelhof.yml").unwrap();
    let deploy_info: DeployInfo = serde_yaml::from_str(&contents).unwrap();

    println!("{}", deploy_info.url);

    spawn_with_output!(bash -c "docker ps").unwrap().wait_with_pipe(&mut |pipe| {
        BufReader::new(pipe)
            .lines()
            .filter_map(|line| line.ok())
            .for_each(|line| println!("{}", line));
    });

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