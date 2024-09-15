// Import necessary crates and modules
use axum::{routing::{get, post}, Router, Json, extract::State};
use sqlx::PgPool; // For managing a connection pool to PostgreSQL
use dotenv::dotenv; // For loading environment variables from a .env file
use std::env; // For accessing environment variables
use chrono::NaiveDate;

mod models;

use models::{Users};


// Main function, entry point of the application
#[tokio::main]
async fn main() {

}