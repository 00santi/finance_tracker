use actix_web::{
    App,
    HttpResponse,
    HttpServer,
    Responder,
    get,
    post,
    web
};
use argon2::password_hash::SaltString;
use argon2::{
    Argon2,
    //PasswordHash,
    PasswordHasher
};
use argon2::password_hash::rand_core::OsRng;
use sqlx;
use sqlx::PgPool;
use serde::{
    Deserialize,
    Serialize
};

mod db;
use db::init_db_pool;
type DynError = Box<dyn std::error::Error + Send + Sync>;

#[actix_web::main]
async fn main() -> Result<(), DynError> {
    let pool = init_db_pool().await?;
    let app_data = web::Data::new(pool);

    HttpServer::new(move ||
        App::new()
            .app_data(app_data.clone())
            .service(health)
            .service(echo)
            .service(create_user))
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
    let pwd_hash = match make_hash(req.password.as_bytes()) {
        Ok(pwd_hash) => pwd_hash,
        Err(e) => return HttpResponse::InternalServerError().body(e),
    };

    let query_result = sqlx::query!(
        r#"
        INSERT INTO users (name, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING name, email, created_at
        "#,
        req.username,
        req.email,
        pwd_hash,
    ).fetch_one(pool.as_ref()).await;

    let row = match query_result {
        Ok(query_result) => query_result,
        Err(e) => {
            if let sqlx::Error::Database(database_error) = e
                && database_error.is_unique_violation() {
                return HttpResponse::Conflict().body("Email already exists");
            }
            return HttpResponse::InternalServerError().body("Error adding user to database");
        }
    };

    let response = UserCreatedResponse {
        name: row.name,
        email: row.email,
        timestamp: row.created_at,
    };

    HttpResponse::Created().json(response)
}

fn make_hash(pwd: &[u8]) -> Result<String, &'static str> {
    let salt = SaltString::generate(&mut OsRng);
    let hashed = Argon2::default().hash_password(pwd, &salt);

    match hashed {
        Err(_) => Err("Error hashing password"),
        Ok(hashed) => Ok(hashed.to_string()),
    }
}

#[derive(Deserialize)]
struct NewUserRequest {
    username: Option<String>,
    email: String,
    password: String,
}

#[derive(Serialize)]
struct UserCreatedResponse {
    name: Option<String>,
    email: String,
    timestamp: chrono::NaiveDateTime,
}
