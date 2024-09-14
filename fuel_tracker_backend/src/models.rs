use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Users {
    pub id: i32,
    pub username: String, // Handle this field as String if necessary
    pub password_hash: String,
}

#[derive(Deserialize)]
pub struct Cars {
    pub car_id: i32,
    pub user_id: i32,
    pub make: String,
    pub model: String,
    pub year: i32,
}
