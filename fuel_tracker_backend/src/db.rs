use sqlx::{ PgPool, Row };
use crate::models::{ User, NewUser };
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

pub async fn add_user(pool: &PgPool, new_user: &NewUser) -> Result<()> {
    sqlx
        ::query(r#"INSERT INTO users (username, password_hash) VALUES ($1, $2)"#)
        .bind(&new_user.username)
        .bind(&new_user.password_hash)
        .execute(pool).await?;

    Ok(())
}
