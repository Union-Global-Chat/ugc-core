use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Message {
    content: String,
    id: String,
    clean_content: String,
    reference: MessageReference,
}

#[derive(Serialize, Deserialize)]
struct MessageReference {
    channnel_id: String,
    guild_id: String,
    message_id: String,
}