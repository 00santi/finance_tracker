use std::env;
use sqlx::PgPool;

pub async fn init_db_pool() -> Result<PgPool, super::DynError> {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let pool = PgPool::connect(url.as_str()).await?;
    Ok(pool)
}
