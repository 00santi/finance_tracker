use dotenvy::dotenv;
use std::env;
use argon2::PasswordHash;
use sqlx::PgPool;

pub async fn init_db_pool() -> Result<PgPool, super::DynError> {
    dotenv()?;
    let url = env::var("DATABASE_URL")?;
    let pool = PgPool::connect(url.as_str()).await?;
    Ok(pool)
}

pub async fn clear_tables(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("TRUNCATE TABLE transactions, users RESTART IDENTITY CASCADE")
        .execute(pool)
        .await?;
    Ok(())
}

pub struct User {
    pub id: usize,
    pub name: Option<String>,
    pub email: String,
    pub password_hash: String,
    pub created_at: String,
}

impl User {
    fn default() -> User {
        User {
            id: 0,
            name: None,
            email: String::new(),
            password_hash: String::new(),
            created_at: String::new(),
        }
    }
}

struct Transaction {
    id: usize,
    user_id: usize,
    category: String,
    description: Option<String>,
    amount: f64,
}

impl Transaction {

}
