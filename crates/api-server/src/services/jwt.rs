use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: u64,
    pub iat: u64,
}

#[derive(Clone)]
pub struct JwtService {
    encoding_key_web: EncodingKey,
    decoding_key_web: DecodingKey,
    encoding_key_game: EncodingKey,
    decoding_key_game: DecodingKey,
    expiration_seconds: u64,
}

impl JwtService {
    pub fn new(secret_web: &str, secret_game: &str, expiration_seconds: u64) -> Self {
        Self {
            encoding_key_web: EncodingKey::from_secret(secret_web.as_bytes()),
            decoding_key_web: DecodingKey::from_secret(secret_web.as_bytes()),
            encoding_key_game: EncodingKey::from_secret(secret_game.as_bytes()),
            decoding_key_game: DecodingKey::from_secret(secret_game.as_bytes()),
            expiration_seconds,
        }
    }

    pub fn sign_with_context(
        &self,
        account_id: Uuid,
        context: TokenContext,
    ) -> Result<String, AppError> {
        let now = jsonwebtoken::get_current_timestamp();
        let iat = now;
        let exp = now + self.expiration_seconds;
        let claims = Claims {
            sub: account_id,
            exp,
            iat,
        };

        let key = match context {
            TokenContext::Web => &self.encoding_key_web,
            TokenContext::Game => &self.encoding_key_game,
        };

        encode(&Header::default(), &claims, key).map_err(|e| AppError::Internal(anyhow::anyhow!(e)))
    }

    pub fn verify_with_context(
        &self,
        token: &str,
        context: TokenContext,
    ) -> Result<Claims, AppError> {
        let key: &DecodingKey = match context {
            TokenContext::Web => &self.decoding_key_web,
            TokenContext::Game => &self.decoding_key_game,
        };

        decode::<Claims>(token, key, &Validation::default())
            .map(|data| data.claims)
            .map_err(|e| match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::Unauthorized,
                _ => AppError::Unauthorized,
            })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TokenContext {
    Web,
    Game,
}

impl std::str::FromStr for TokenContext {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "web" => Ok(TokenContext::Web),
            "game" => Ok(TokenContext::Game),
            _ => Err(()),
        }
    }
}
