use crate::errors::errors::Error;

use anyhow::Ok;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

impl Claims {
    pub fn new(user_id: String) -> Result<String, Error> {
        let claim = Self {
            sub: user_id,
            exp: Utc::now().timestamp() as usize + Duration::minutes(1).num_seconds() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let header = Header::default();
        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set.");

        encode(
            &header,
            &claim,
            &EncodingKey::from_secret(&secret.as_bytes()),
        )
        .map_err(|_| Error::JwtError("Failed to create token.".to_string()))
    }

    // pub fn is_valid(token: &str) -> Result<Self, Error> {
    //     let secret = std::env::var("JWT_SECRET")
    //         .expect("JWT_SECRET must be set.")
    //         .as_bytes();

    //     decode::<Self>(
    //         token,
    //         &DecodingKey::from_secret(secret),
    //         &Validation::default(),
    //     )
    //     .map_err(|_| Error::JwtError("Failed to create token.".to_string()));
    // }
}
