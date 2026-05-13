use actix_web::{
    HttpResponse,
    Responder,
    post,
    web
};
use argon2::{
    password_hash::PasswordVerifier,
    Argon2,
    PasswordHash,
};
use sqlx::{
    self,
    PgPool,
};
use serde::{
    Deserialize,
    Serialize
};
use jsonwebtoken::{
    encode, 
    EncodingKey, 
    Header
};

const EXPIRATION_TIME: i64 = 604800;

#[post("/login")]
pub async fn login(pool: web::Data<PgPool>, req: web::Json<LoginRequest>) -> impl Responder {
    let error401 = HttpResponse::Unauthorized().body("Email doesn't exist, or password is incorrect");

    let query_result = sqlx::query!(
        "SELECT id, email, password_hash FROM users WHERE email = $1",
        req.email
    ).fetch_one(pool.as_ref()).await;

    let row = match query_result {
        Ok(query_result) => query_result,
        Err(_) => return error401,
    };

    let given = req.password.as_bytes();
    let saved = match PasswordHash::new(row.password_hash.as_str()) {
        Ok(x) => x,
        Err(_) => return error401
    };

    if Argon2::default().verify_password(given, &saved).is_err() {
        return error401;
    }

    let claims = Claims {
        sub: row.id,
        exp: chrono::Utc::now().timestamp() + EXPIRATION_TIME
    };

    let access_token = match encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_ref()),
    ) {
        Ok(t) => t,
        _ => return error401
    };

    let response = LoginResponse {
        access_token,
        token_type: "Bearer".to_string(),
    };

    HttpResponse::Ok().json(response)
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: i32,
    exp: i64,
}

#[derive(Serialize, Deserialize)]
struct LoginResponse {
    access_token: String,
    token_type: String,
}
