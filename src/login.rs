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
use sqlx;
use serde::{
    Deserialize,
    Serialize
};
use jsonwebtoken::{
    encode,
    EncodingKey,
    Header
};
use crate::{AppState, valid_email, valid_pwd};
use crate::auth::Claims;

const EXPIRATION_TIME: i64 = 604800;

#[post("/login")]
pub async fn login(state: web::Data<AppState>, req: web::Json<LoginRequest>) -> impl Responder {
    let error401 = HttpResponse::Unauthorized().body("Email doesn't exist, or password is incorrect");

    if !valid_email(&req.email) || !valid_pwd(&req.password) {
        return error401;
    }

    if  !(6..=255).contains(&req.email.len()) ||
        !(8..=64).contains(&req.password.len()) {
        return error401;
    }

    let query_result = sqlx::query!(
        "SELECT id, email, password_hash FROM users WHERE email = $1",
        req.email
    ).fetch_one(&state.pool).await;

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
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
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

#[derive(Serialize)]
struct LoginResponse {
    access_token: String,
    token_type: String,
}
