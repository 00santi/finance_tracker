use actix_web::{
    HttpResponse,
    Responder,
    post,
    web
};
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

#[post("/transaction")]
pub async fn transaction(pool: web::Data<PgPool>, req: web::Json<TransRequest>) -> impl Responder {

    HttpResponse::Ok()
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
