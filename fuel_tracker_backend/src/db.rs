use sqlx::PgPool;
use sqlx::Result;

pub async fn is_up(pool: &PgPool) -> Result<String> {
    match sqlx::query("SELECT 1").execute(pool).await {
        Ok(_) => Ok("A.P.I is up!".to_string()),
        Err(_) => Ok("DB is Down but A.P.I is up".to_string()),
    }
}
