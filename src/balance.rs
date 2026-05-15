use crate::auth::extract_claims;
use crate::AppState;
use actix_web::{HttpResponse, Responder, web, HttpRequest, get};
use sqlx;
use serde::Serialize;
use sqlx::types::BigDecimal;

#[get("/balance")]
pub async fn get(state: web::Data<AppState>, http_request: HttpRequest) -> impl Responder {
    let claims = match extract_claims(http_request, state.jwt_secret.as_bytes()) {
        Ok(c) => c,
        Err(e) => return e,
    };

    let balance: BigDecimal = match
    sqlx::query_scalar!("SELECT COALESCE(SUM(amount), 0) FROM transactions WHERE user_id = $1;", claims.sub)
        .fetch_one(&state.pool).await {
        Ok(d) => d.unwrap_or_default(),
        _ => return HttpResponse::InternalServerError().body("Error fetching transactions"),
    };
    let balance = balance.to_plain_string();
    HttpResponse::Ok().json(GetResponse {balance})
}

#[derive(Serialize)]
struct GetResponse {
    balance: String,
}
