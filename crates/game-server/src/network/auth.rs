use bevy::prelude::Resource;
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    pub sub: Uuid,
    pub exp: u64,
    pub iat: u64,
}

#[derive(Debug, Clone, Resource)]
pub struct GameTokenVerifier {
    decoding_key: DecodingKey,
}

impl GameTokenVerifier {
    pub fn from_secret(secret: &str) -> Self {
        Self {
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
        }
    }

    pub fn verify_account_id(&self, token: &str) -> Result<Uuid, String> {
        decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map(|data| data.claims.sub)
            .map_err(|err| err.to_string())
    }
}
