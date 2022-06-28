use actix_web::{get, post, web, Responder};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Message {
    content: String,
    id: String,
    clean_content: String,
    reference: Option<MessageReference>,
}

#[derive(Serialize, Deserialize)]
struct MessageReference {
    channnel_id: String,
    guild_id: String,
    message_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseContent {
    code: String,
    message: String,
    data: Option<serde_json::Value>,
}

#[post("/api/messages")]
pub async fn post_message(message: web::Json<Message>) -> impl Responder {
    println!("{}", message.content);
    web::Json(ResponseContent {
        code: "200".to_string(),
        message: "OK".to_string(),
        data: None,
    })
}

#[get("/api/messages")]
pub async fn get_messages() -> impl Responder {
    "Hello some one."
}