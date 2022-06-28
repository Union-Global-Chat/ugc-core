use actix_web::{get, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

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
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(main::index)
            .service(message::post_message)
            .service(message::get_messages)
    })
    .bind("localhost:8080")?
    .run()
    .await
}