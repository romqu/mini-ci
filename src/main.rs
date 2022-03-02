#![feature(once_cell)]
#![feature(map_try_insert)]
extern crate lazy_static;
extern crate regex;

use actix_web::{App, HttpServer, web};

use untitled::entrypoint::post_github_push_event_handler::handle_post_github_push_event;
use untitled::InitError;

#[tokio::main]
async fn main() -> Result<(), InitError> {

    /*let client = Client::builder().build();

    let github_user_info = client
        .get("https://api.github.com/user")
        .header("User-Agent", "request")
        .header("accept", "*")
        .bearer_auth("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")
        .send()
        .await
        .unwrap()
        .json::<GithubUserInfo>()
        .await
        .unwrap();

    println!("{}", github_user_info.name);*/

    /*let a = reqwest::get("https://raw.githubusercontent.com/romqu/ubuntu-config-scripts/master/README.md?token=GHSAT0AAAAAABR6TMBHGSVKHW4HAVLNYJIYQZGTZA")
    .await.unwrap().status();*/

    /*    match init_app() {
        Ok(_) => start_app().await.map_err(|_| CouldNotStartApp),
        Err(err) => Err(err),
    }
*/

    Ok({})
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

