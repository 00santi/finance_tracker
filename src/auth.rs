use actix_web::{HttpRequest, HttpResponse};
use chrono::Utc;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: i64,
}

pub fn extract_claims(req: HttpRequest, jwt_secret: &[u8]) -> Result<Claims, HttpResponse> {
    let token_error = Err(HttpResponse::Unauthorized().body("Invalid token"));
    
    let auth_token = match req.headers().get("Authorization") {
        Some(auth) => auth,
        _ => return token_error
    };
    
    let auth_token = match auth_token.to_str() {
        Ok(s) => s,
        _ => return token_error
    };
    
    if !auth_token.starts_with("Bearer ") {
        return token_error;
    }

    let decoded = decode::<Claims>(
        &auth_token[7..],
        &DecodingKey::from_secret(jwt_secret),
        &Validation::default(),
    );

    let token_data = match decoded {
        Ok(d) => d,
        _ => return token_error
    };

    if Utc::now().timestamp() > token_data.claims.exp {
        return Err(HttpResponse::Unauthorized().body("Expired token, sign in again"))
    }
    
    Ok(token_data.claims)
}