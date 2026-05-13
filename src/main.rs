use actix_web::{
    App,
    HttpResponse,
    HttpServer,
    Responder,
    get,
    post,
    web
};
use sqlx::PgPool;

mod db;
mod create_user;
mod login;
mod transactions;
mod auth;

type DynError = Box<dyn std::error::Error + Send + Sync>;

#[actix_web::main]
async fn main() -> Result<(), DynError> {
    let pool = db::init_db_pool().await?;
    let jwt_secret = std::env::var("JWT_SECRET")?;
    let app_data = web::Data::new(AppState { pool, jwt_secret });

    HttpServer::new(move ||
        App::new()
            .app_data(app_data.clone())
            .service(health)
            .service(echo)
            .service(create_user::post)
            .service(login::post)
            .service(transactions::post)
            .service(transactions::get))
            .bind(("127.0.0.1", 7878))?
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

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

fn valid_email(email: &str) -> bool {
    !(6..=255).contains(&email.len())
}

fn valid_pwd(pwd: &str) -> bool {
    !(6..=255).contains(&pwd.len())
}
