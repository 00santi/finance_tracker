use actix_web::
{
    get,
    post,
    App,
    HttpResponse,
    HttpServer,
    Responder
};
use sqlx;
mod db;
type DynError = Box<dyn std::error::Error + Send + Sync>;

#[actix_web::main]
async fn main() -> Result<(), DynError> {
    let pool = db::init_db_pool().await?;
    sqlx::query("SELECT 1").execute(&pool).await?;

    return Ok(());
    HttpServer::new(|| {
        App::new()
            .service(health)
            .service(echo)
    })
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
