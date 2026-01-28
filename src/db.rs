use dotenvy::dotenv;
use std::env;
use sqlx::PgPool;

pub async fn init_db_pool() -> Result<PgPool, super::DynError> {
    dotenv()?;
    let url = env::var("DATABASE_URL")?;
    let pool = PgPool::connect(url.as_str()).await?;
    Ok(pool)
}
