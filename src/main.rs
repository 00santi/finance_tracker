use actix_web::{
    App,
    HttpResponse,
    HttpServer,
    Responder,
    get,
    post,
    web
};
use actix_cors::Cors;
use sqlx::PgPool;

mod db;
mod register;
mod login;
mod transactions;
mod auth;
mod balance;

type DynError = Box<dyn std::error::Error + Send + Sync>;

#[actix_web::main]
async fn main() -> Result<(), DynError> {
    let pool = db::init_db_pool().await?;
    let jwt_secret = std::env::var("JWT_SECRET")?;
    let host = std::env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or("7878".to_string()).parse()?;
    let app_data = web::Data::new(AppState { pool, jwt_secret });
    let frontend_origin = "http://localhost:5173";

    HttpServer::new(move ||
        App::new()
            .app_data(app_data.clone())
            .service(homepage)
            .service(health)
            .service(echo)
            .service(register::post)
            .service(login::post)
            .service(transactions::post)
            .service(transactions::get)
            .service(balance::get)
            .wrap(Cors::default()
                .allowed_origin(frontend_origin)
                .allow_any_header()
                .allow_any_method()
            ))
            .bind((host, port))?
            .run()
            .await?;

    Ok(())
}

pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: String,
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/")]
async fn homepage() -> impl Responder {
    HttpResponse::Ok()
        .body("Finance Tracker :D check the GitHub README for usage https://github.com/00santi/finance_tracker")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/clear_db")]
async fn clear_db(state: web::Data<AppState>) -> impl Responder {
    let q = sqlx::query("TRUNCATE TABLE transactions, users RESTART IDENTITY CASCADE")
        .execute(&state.pool)
        .await;

    if let Err(_) = q {
        HttpResponse::Ok().body("error")
    } else {
        HttpResponse::Ok().body("db cleared")
    }
}

fn valid_email(email: &str) -> bool {
    (6..=255).contains(&email.len())
}

fn valid_pwd(pwd: &str) -> bool {
    (6..=255).contains(&pwd.len())
}
