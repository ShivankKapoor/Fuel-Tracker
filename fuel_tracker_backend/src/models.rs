use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
pub struct Car {
    pub car_id: i32,
    pub user_id: i32,
    pub make: String,
    pub model: String,
    pub year: i32,
}

#[derive(Serialize, Deserialize)]
pub struct NewCar {
    pub user_id: i32,
    pub make: String,
    pub model: String,
    pub year: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CarCheckIn {
    pub checkin_id: i32,
    pub car_id: i32,
    pub checkin_date: NaiveDate,
    pub miles: i32,
    pub tank_left: i32,
}

#[derive(Serialize, Deserialize)]
pub struct NewCarCheckIn {
    pub car_id: i32,
    pub checkin_date: NaiveDate,
    pub miles: i32,
    pub tank_left: i32,
}

#[derive(Serialize, Deserialize)]
pub struct FillUp {
    pub fillup_id: i32,
    pub car_id: i32,
    pub fillup_date: String,
    pub gallons_filled: f64,
    pub price_per_gallon: f64,
    pub new_range: i32,
}

#[derive(Serialize, Deserialize)]
pub struct NewFillUp {
    pub car_id: i32,
    pub fillup_date: String,
    pub gallons_filled: f64,
    pub price_per_gallon: f64,
    pub new_range: i32,
}
