use actix_web::{App, HttpResponse, HttpServer, Responder, get, post};
use sqlx;
mod db;
use db::{User, clear_tables, init_db_pool};
type DynError = Box<dyn std::error::Error + Send + Sync>;

#[actix_web::main]
async fn main() -> Result<(), DynError> {
    let db_pool = init_db_pool().await?;
    let pool = &db_pool;
    clear_tables(pool).await?;

    let email = "test@test.com".to_string();
    let password_hash = "123".to_string();

    sqlx::query!(
        r#"
        INSERT INTO users (email, password_hash)
        VALUES ($1, $2)
        "#,
        email,
        password_hash
    ).execute(pool).await?;

    let row = sqlx::query!(
        r#"
            SELECT id, name, email, password_hash, created_at
            FROM users
            WHERE email = $1
        "#,
        email
    ).fetch_one(pool).await?;

    println!(
        "User id: {}, name: {:?}, email: {}, created_at: {:?}",
        row.id, row.name, row.email, row.created_at
    );

    Ok(())

    /*HttpServer::new(|| App::new().service(health).service(echo))
        .bind(("127.0.0.1", 7878))?
        .run()
        .await?;

    Ok(())*/
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
