use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Users {
    pub id: i32,
    pub username: String, // Handle this field as String if necessary
    pub password_hash: String,
}

#[derive(Deserialize)]
pub struct NewMessage {
    pub message: String,
}
