use actix_web::{get, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct IndexContent {
    message: String,
}

#[get("/")]
async fn greet() -> impl Responder {
    let content = IndexContent {
        message: "This is api core.".to_string(),
    };
    web::Json(content)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}