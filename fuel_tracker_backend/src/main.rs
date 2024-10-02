use axum::{ extract::State, response::IntoResponse, routing::{ get, post }, Json, Router };
use sqlx::PgPool;
use dotenv::dotenv;
use std::env;
use chrono::NaiveDate;

mod models;
mod db;

use models::{ User, NewUser, Cars, CarCheckIns, FillUps };
use db::{ fetch_all_users, is_up, add_user };

pub async fn is_api_up(State(pool): State<PgPool>) -> Json<String> {
    let status = is_up(&pool).await.unwrap_or_else(|_| "Failed to retrieve status".to_string());
    Json(status)
}

pub async fn get_all_users(State(pool): State<PgPool>) -> Json<Vec<User>> {
    match fetch_all_users(&pool).await {
        Ok(users) => Json(users),
        Err(_) => Json(vec![]),
    }
}

pub async fn create_user(
    State(pool): State<PgPool>,
    Json(user): Json<NewUser>
) -> impl IntoResponse {
    match add_user(&pool, &user).await {
        Ok(_) => (axum::http::StatusCode::CREATED, "User created").into_response(),
        Err(_) =>
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create user",
            ).into_response(),
    }
}

// Main function, entry point of the application
#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("Failed to connect to DB");

    let app = Router::new()
        .route("/", get(is_api_up))
        .route("/users", get(get_all_users))
        .route("/create-user",post(create_user))
        .with_state(pool);

    axum::Server
        ::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service()).await
        .unwrap();
}
