use actix_web::{
    HttpResponse,
    Responder,
    post,
    web
};
use actix_web::error::HttpError;
use argon2::password_hash::PasswordVerifier;

use argon2::{
    Argon2,
    PasswordHash,
};
use sqlx;
use sqlx::PgPool;
use serde::{
    Deserialize,
    Serialize
};

use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::types::BigDecimal;

const VALID_CATEGORIES: [&str; 6] = ["PERSONAL", "BUSINESS", "PAYCHECK", "TRAVEL", "RENT", "GROCERIES"];

#[post("/transaction")]
pub async fn transaction(pool: web::Data<PgPool>, req: web::Json<TransRequest>) -> impl Responder {
    // validate request
    // validate token
    // use token to determine user_id
    let user_id = 0;

    let category = req.category.trim().to_uppercase();
    if !VALID_CATEGORIES.contains(&category.as_str()) {
        return HttpResponse::BadRequest().body("Invalid category")
    };

    let amount = match BigDecimal::try_from(req.amount) {
        Ok(a) => a,
        _ => return HttpResponse::BadRequest().body("Invalid amount"),
    };

    if sqlx::query!(
        "INSERT INTO transactions (user_id, amount, category, description) VALUES ($1, $2, $3, $4)",
        user_id, amount, req.category, req.description,
    ).fetch_one(pool.as_ref()).await
        .is_err() {
        return HttpResponse::BadRequest().body("Invalid transaction")
    }

    HttpResponse::Ok().body("")
}

#[derive(Deserialize)]
struct TransRequest {
    amount: f64,
    category: String,
    description: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct TransResponse {

}
