use crate::auth::extract_claims;
use crate::AppState;
use actix_web::{HttpResponse, Responder, post, web, HttpRequest, get};
use sqlx;
use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;

const VALID_CATEGORIES: [&str; 6] = ["PERSONAL", "BUSINESS", "PAYCHECK", "TRAVEL", "RENT", "GROCERIES"];

fn valid_body(req: &web::Json<PostRequest>) -> bool {
    if req.category.len() > 50 {
        false
    } else if let Some(desc) = &req.description && desc.len() > 50  {
        false
    } else {
        true
    }
}

#[post("/transactions")]
pub async fn post(state: web::Data<AppState>, req_body: web::Json<PostRequest>, http_request: HttpRequest) -> impl Responder {
    if !valid_body(&req_body) {
        return HttpResponse::BadRequest().body("Invalid request");
    }

    let claims = match extract_claims(http_request, state.jwt_secret.as_bytes()) {
        Ok(c) => c,
        Err(e) => return e,
    };

    let category = req_body.category.trim().to_uppercase();
    if !VALID_CATEGORIES.contains(&category.as_str()) {
        return HttpResponse::BadRequest().body("Invalid category")
    };

    let amount = match BigDecimal::try_from(req_body.amount) {
        Ok(a) => a,
        _ => return HttpResponse::BadRequest().body("Invalid amount"),
    };

    if sqlx::query!(
        "INSERT INTO transactions (user_id, amount, category, description) VALUES ($1, $2, $3, $4)",
        claims.sub, amount, category, req_body.description,
    ).execute(&state.pool).await
        .is_err() {
        return HttpResponse::BadRequest().body("Invalid transaction")
    }

    HttpResponse::Ok().body("")
}

#[get("/transactions")]
pub async fn get(state: web::Data<AppState>, http_request: HttpRequest) -> impl Responder {
    let claims = match extract_claims(http_request, state.jwt_secret.as_bytes()) {
        Ok(c) => c,
        Err(e) => return e,
    };

    let rows: Vec<_> = match
    sqlx::query!("SELECT amount, category, description, created_at FROM transactions WHERE user_id = $1", claims.sub)
        .fetch_all(&state.pool).await {
        Ok(vec) => vec,
        _ => return HttpResponse::InternalServerError().body("Error fetching transactions"),
    };

    let mut transactions: Vec<GetResponse> = Vec::with_capacity(rows.len());
    for r in rows {
        let amount = BigDecimal::to_plain_string(&r.amount);
        let category = r.category;
        let description = r.description;
        let created_at = r.created_at;
        transactions.push(GetResponse {amount, category, description, created_at});
    }

    HttpResponse::Ok().json(transactions)
}

#[derive(Deserialize)]
struct PostRequest {
    amount: f64,
    category: String,
    description: Option<String>,
}

#[derive(Serialize)]
struct GetResponse {
    amount: String,
    category: String,
    description: Option<String>,
    created_at: chrono::NaiveDateTime,
}
