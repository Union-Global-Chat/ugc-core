use actix_web::{get, Responder, web};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct IndexContent {
    message: String,
}

#[get("/api")]
pub async fn index() -> impl Responder {
    let content = IndexContent {
        message: "Hello some one.".to_string(),
    };
    web::Json(content)
}