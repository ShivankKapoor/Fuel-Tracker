use serde::{ Deserialize, Serialize };
use chrono::NaiveDate;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
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

#[derive(Deserialize)]
pub struct CarCheckIns {
    pub checkin_id: i32,
    pub car_id: i32,
    pub checkin_date: NaiveDate,
    pub miles: i32,
    pub tank_left: i32,
}

#[derive(Deserialize)]
pub struct FillUps {
    pub fillup_id: i32,
    pub car_id: i32,
    pub fillup_date: NaiveDate,
    pub gallons_filled: f64,
    pub price_per_gallon: f64,
    pub new_range: i32,
}
