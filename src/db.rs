use dotenvy::dotenv;
use std::env;
//use argon2::PasswordHash;
//use serde::Serialize;
use sqlx::PgPool;

pub async fn init_db_pool() -> Result<PgPool, super::DynError> {
    dotenv()?;
    let url = env::var("DATABASE_URL")?;
    let pool = PgPool::connect(url.as_str()).await?;
    Ok(pool)
}

pub async fn _clear_tables(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("TRUNCATE TABLE transactions, users RESTART IDENTITY CASCADE")
        .execute(pool)
        .await?;
    Ok(())
}
