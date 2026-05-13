use crate::auth::extract_claims;
use crate::AppState;
use actix_web::{HttpResponse, Responder, post, web, HttpRequest};
use sqlx;
use serde::Deserialize;

use sqlx::types::BigDecimal;

const VALID_CATEGORIES: [&str; 6] = ["PERSONAL", "BUSINESS", "PAYCHECK", "TRAVEL", "RENT", "GROCERIES"];

fn valid_req(req: &web::Json<TransRequest>) -> bool {
    if req.category.len() > 50 {
        return false;
    }

    if let Some(desc) = &req.description && desc.len() > 50 {
        return false;
    }

    true
}

#[post("/transactions")]
pub async fn transactions(state: web::Data<AppState>, request_body: web::Json<TransRequest>, http_request: HttpRequest) -> impl Responder {
    if !valid_req(&request_body) {
        return HttpResponse::BadRequest().body("Invalid request");
    }

    let claims = match extract_claims(http_request, state.jwt_secret.as_bytes()) {
        Ok(c) => c,
        Err(e) => return e,
    };

    let category = request_body.category.trim().to_uppercase();
    if !VALID_CATEGORIES.contains(&category.as_str()) {
        return HttpResponse::BadRequest().body("Invalid category")
    };

    let amount = match BigDecimal::try_from(request_body.amount) {
        Ok(a) => a,
        _ => return HttpResponse::BadRequest().body("Invalid amount"),
    };

    if sqlx::query!(
        "INSERT INTO transactions (user_id, amount, category, description) VALUES ($1, $2, $3, $4)",
        claims.sub, amount, category, request_body.description,
    ).execute(&state.pool).await
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
