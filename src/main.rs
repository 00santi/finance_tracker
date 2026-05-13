use actix_web::{
    App,
    HttpResponse,
    HttpServer,
    Responder,
    get,
    post,
    web
};

mod db;
mod create_user;
mod login;
mod transaction;

type DynError = Box<dyn std::error::Error + Send + Sync>;

#[actix_web::main]
async fn main() -> Result<(), DynError> {
    let pool = db::init_db_pool().await?;
    let app_data = web::Data::new(pool);

    HttpServer::new(move ||
        App::new()
            .app_data(app_data.clone())
            .service(health)
            .service(echo)
            .service(create_user::create_user)
            .service(login::login)
            .service(transaction::transaction))
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

fn valid_email(email: &str) -> bool {
    !(6..=255).contains(&email.len())
}

fn valid_pwd(pwd: &str) -> bool {
    !(6..=255).contains(&pwd.len())
}
