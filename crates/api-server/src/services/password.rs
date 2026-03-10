use bcrypt::{DEFAULT_COST, hash, verify};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("Failed to hash password")]
    HashFailed,

    #[error("Failed to verify password")]
    VerifyFailed,
}

#[derive(Clone)]
pub struct PasswordHasher {
    cost: u32,
}

impl PasswordHasher {
    pub fn new(cost: u32) -> Self {
        Self { cost }
    }

    pub fn hash(&self, password: &str) -> Result<String, PasswordError> {
        hash(password, self.cost).map_err(|_| PasswordError::HashFailed)
    }

    pub fn verify(&self, password: &str, hash: &str) -> Result<bool, PasswordError> {
        verify(password, hash).map_err(|_| PasswordError::VerifyFailed)
    }
}

impl Default for PasswordHasher {
    fn default() -> Self {
        Self::new(DEFAULT_COST)
    }
}
