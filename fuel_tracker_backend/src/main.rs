use axum::{ routing::{ get, post }, Router, Json, extract::State };
use sqlx::PgPool;
use dotenv::dotenv;
use std::env;
use chrono::NaiveDate;

mod models;
mod db;

use models::{ Users, Cars, CarCheckIns, FillUps };
use db::{is_up}; 


pub async fn is_api_up(State(pool): State<PgPool>) -> Json<String> {
    let messages = is_up(&pool).await.unwrap_or_else(|_| "Failed to retrieve messages".to_string());
    Json(messages)
}

// Main function, entry point of the application
#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("Failed to connect to DB");

    let app = Router::new()
        .route("/", get(is_api_up))
        .with_state(pool);

    axum::Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
