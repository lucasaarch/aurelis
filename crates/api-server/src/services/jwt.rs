use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: u64,
}

#[derive(Clone)]
pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    expiration_seconds: u64,
}

impl JwtService {
    pub fn new(secret: &str, expiration_seconds: u64) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
            expiration_seconds,
        }
    }

    pub fn sign(&self, account_id: Uuid) -> Result<String, AppError> {
        let exp = jsonwebtoken::get_current_timestamp() + self.expiration_seconds;
        let claims = Claims { sub: account_id, exp };

        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|e| AppError::Internal(anyhow::anyhow!(e)))
    }

    pub fn verify(&self, token: &str) -> Result<Claims, AppError> {
        decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map(|data| data.claims)
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::Unauthorized,
                _ => AppError::Unauthorized,
            })
    }
}
