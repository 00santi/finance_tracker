use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher};
use argon2::password_hash::rand_core::OsRng;
use sqlx;
use sqlx::PgPool;
use sqlx::types::BigDecimal;
use num_traits::cast::FromPrimitive;
use serde::Deserialize;

mod db;
use db::{User, clear_tables, init_db_pool};
type DynError = Box<dyn std::error::Error + Send + Sync>;

#[actix_web::main]
async fn main() -> Result<(), DynError> {
    let pool = init_db_pool().await?;

    HttpServer::new(|| App::new().service(health).service(echo))
        .bind(("127.0.0.1", 7878))?
        .run()
        .await?;

    Ok(())
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/users")]
async fn create_user(pool: web::Data<PgPool>, req: web::Json<NewUserRequest>) -> impl Responder {
    let pwd = make_hash(req.password.clone());
    insert_new_user(req.username.clone().unwrap_or_default(), req.email.clone(), pwd, pool).await;
    HttpResponse::Ok().body("User added successfully")
}

async fn insert_new_user(name: String, email: String, password_hash: String, pool: web::Data<PgPool>) -> User {
    sqlx::query!(
        r#"
        INSERT INTO users (email, password_hash)
        VALUES ($1, $2)
        "#,
        email,
        password_hash
    ).execute(pool.get_ref()).await.unwrap();
    
    let row = sqlx::query!(
        r#"
            SELECT id, name, email, password_hash, created_at
            FROM users
            WHERE email = $1
        "#,
        email
    ).fetch_one(pool.get_ref()).await.unwrap();
    
    User {
        id: row.id as usize,
        name: row.name,
        email: row.email,
        password_hash: row.password_hash,
        created_at: String::from("placeholder_time"),
    }
}

fn make_hash(pwd: String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default().hash_password(pwd.as_bytes(), &salt).unwrap().to_string()
}

#[derive(Deserialize)]
struct NewUserRequest {
    username: Option<String>,
    email: String,
    password: String,
}


/*
async fn test_insert_user(pool: &PgPool) {
    let email = "test@test.com".to_string();
    let password_hash = "123".to_string();

    sqlx::query!(
        r#"
        INSERT INTO users (email, password_hash)
        VALUES ($1, $2)
        "#,
        email,
        password_hash
    ).execute(pool).await.unwrap();

    let row = sqlx::query!(
        r#"
            SELECT id, name, email, password_hash, created_at
            FROM users
            WHERE email = $1
        "#,
        email
    ).fetch_one(pool).await.unwrap();

    println!(
        "User id: {}, name: {:?}, email: {}, created_at: {:?}",
        row.id, row.name, row.email, row.created_at
    );
}

async fn test_insert_trans(pool: &PgPool) {
    let user_id = 1;
    let amount = BigDecimal::from_f64(12.34).unwrap();
    let category = "mmhmm";

    sqlx::query!(
        r#"
        INSERT INTO transactions (user_id, amount, category)
        VALUES ($1, $2, $3)
        "#,
        user_id,
        amount,
        category,
    ).execute(pool).await.unwrap();

    let row = sqlx::query!(
        r#"
            SELECT id, user_id, amount, category, description, created_at
            FROM transactions
            WHERE user_id = $1
        "#,
        user_id
    ).fetch_one(pool).await.unwrap();

    println!("Transaction id: {:?}, User id: {:?}, amount: {:?}, category: {:?}, description: {:?},  created at: {:?}",
             row.id, row.user_id, row.amount, row.category, row.description, row.created_at);
}
 */