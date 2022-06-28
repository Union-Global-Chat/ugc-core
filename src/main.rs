use actix_web::{get, web, App, HttpServer, Responder};
use actix_web::middleware::Logger;
use serde::{Deserialize, Serialize};
use env_logger::Env;

pub mod api;
use crate::api::{main, message};

#[derive(Serialize, Deserialize)]
struct IndexContent {
    message: String,
}

#[get("/")]
async fn index() -> impl Responder {
    let content = IndexContent {
        message: "This is api core.".to_string(),
    };
    web::Json(content)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(main::index)
            .service(message::post_message)
            .service(message::get_messages)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind("localhost:8080")?
    .run()
    .await
}