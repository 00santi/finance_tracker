use actix_web::{
    post,
    web,
    HttpResponse,
    Responder
};
use argon2::{
    Argon2,
    PasswordHasher,
    password_hash::rand_core::OsRng,
    password_hash::SaltString,
};
use serde::{
    Deserialize,
    Serialize
};
pub use sqlx;
use crate::{valid_email, valid_pwd, AppState};

#[post("/users")]
async fn post(state: web::Data<AppState>, req: web::Json<NewUserRequest>) -> impl Responder {

    if !valid_email(&req.email) || !valid_pwd(&req.password) {
        return HttpResponse::BadRequest().body("Invalid request");
    }

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
    ).fetch_one(&state.pool).await;

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