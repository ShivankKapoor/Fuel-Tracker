use sqlx::{ PgPool, Row };
use crate::models::{ Car, NewCar, NewFillUp, NewUser, User };
use sqlx::Result;

pub async fn is_up(pool: &PgPool) -> Result<String> {
    match sqlx::query("SELECT 1").execute(pool).await {
        Ok(_) => Ok("A.P.I is up!".to_string()),
        Err(_) => Ok("DB is Down but A.P.I is up".to_string()),
    }
}

pub async fn fetch_all_users(pool: &PgPool) -> Result<Vec<User>> {
    let users = sqlx
        ::query(r#"SELECT user_id, username, password_hash FROM users"#)
        .fetch_all(pool).await?;

    let users = users
        .into_iter()
        .map(|row| User {
            id: row.get("user_id"),
            username: row.get("username"),
            password_hash: row.get("password_hash"),
        })
        .collect();

    Ok(users)
}

pub async fn fetch_all_cars(pool: &PgPool) -> Result<Vec<Car>> {
    let cars = sqlx
        ::query(r#"SELECT car_id, user_id, make,model,year FROM cars"#)
        .fetch_all(pool).await?;

    let cars = cars
        .into_iter()
        .map(|row| Car {
            car_id: row.get("car_id"),
            user_id: row.get("user_id"),
            make: row.get("make"),
            model: row.get("model"),
            year: row.get("year"),
        })
        .collect();

    Ok(cars)
}

pub async fn add_user(pool: &PgPool, new_user: &NewUser) -> Result<()> {
    sqlx
        ::query(r#"INSERT INTO users (username, password_hash) VALUES ($1, $2)"#)
        .bind(&new_user.username)
        .bind(&new_user.password_hash)
        .execute(pool).await?;

    Ok(())
}

pub async fn add_car(pool: &PgPool, new_car: &NewCar) -> Result<()> {
    sqlx
        ::query(r#"INSERT INTO cars (user_id, make, model, year) VALUES ($1, $2, $3, $4)"#)
        .bind(&new_car.user_id)
        .bind(&new_car.make)
        .bind(&new_car.model)
        .bind(&new_car.year)
        .execute(pool).await?;

    Ok(())
}

pub async fn add_fillup(pool: &PgPool, new_fillup: &NewFillUp) -> Result<()>{
    sqlx
        ::query(r#"INSERT INTO fillups (car_id, fillup_date, gallons_filled, price_per_gallon, new_range) VALUES ($1, $2, $3, $4, $5)"#)
        .bind(&new_fillup.car_id)
        .bind(&new_fillup.fillup_date)
        .bind(&new_fillup.gallons_filled)
        .bind(&new_fillup.price_per_gallon)
        .bind(&new_fillup.new_range)
        .execute(pool).await?;

    Ok(())
}
