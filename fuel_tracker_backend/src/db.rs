use sqlx::PgPool;
use crate::models::{Users, Cars, CarCheckIns, FillUps};
use sqlx::Result; 

pub async fn is_up(pool: &PgPool) -> Result<String> {
    let result = sqlx::query("SELECT 1")
        .execute(pool)
        .await;

    match result {
        Ok(_) => Ok("A.P.I is up!".to_string()),
        Err(err) => Err(err),
    }
}