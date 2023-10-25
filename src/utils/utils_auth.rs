use crate::errors::errors::Error;
use crate::models::model_claim::Claims;

use axum::http::{HeaderMap, HeaderValue, StatusCode};

pub async fn set_jwt_cookie(user_id: String) -> Result<HeaderMap, Error> {
    let token =
        Claims::new(user_id).map_err(|e| Error::JwtError("Invalid credentials.".to_string()))?;

    let mut response_headers = HeaderMap::new();

    match HeaderValue::from_str(&format!("token={}", token)) {
        Ok(cookie) => {
            response_headers.insert("Set-Cookie", cookie);
        }
        Err(_) => return Err(Error::JwtError("Failed to set cookie.".to_string())),
    }

    return Ok(response_headers);
}

pub async fn decode_jwt_cookie(token: String) -> Result<Claims, Error> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

    let decoded = jsonwebtoken::decode::<Claims>(
        &token,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
        &jsonwebtoken::Validation::default(),
    )
    .map_err(|_| Error::JwtError("Failed to decode token.".to_string()))?;

    return Ok(decoded.claims);
}
