#![feature(once_cell)]
#![feature(map_try_insert)]
extern crate lazy_static;
extern crate regex;

use std::fmt::Debug;

use actix_web::{App, HttpServer, web};
use git2::{ObjectType, Repository, Tree};

use untitled::{init_app, InitError};
use untitled::entrypoint::post_github_push_event_handler::handle_post_github_push_event;

#[tokio::main]
async fn main() -> Result<(), InitError> {
    let repo = Repository::open("/home/roman/projects/private/mini-ci").unwrap();
    let object = repo.revparse_single("master").unwrap();


    match object.kind() {
        Some(ObjectType::Commit) => {
            show_tree(&object.as_commit().unwrap().tree().unwrap());
        }
        _ => {}
    }


    init_app().await?;

    Ok({})
}

fn show_tree(tree: &Tree) {
    for entry in tree.iter() {
        if entry.name().unwrap() == "deploy.sh" {
            println!("{}", entry.id());
        }
    }
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

