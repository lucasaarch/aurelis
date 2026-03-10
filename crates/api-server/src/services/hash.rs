use bcrypt::{DEFAULT_COST, hash, verify};

use crate::error::AppError;

#[derive(Clone)]
pub struct HashService {
    cost: u32,
}

impl HashService {
    pub fn new(cost: u32) -> Self {
        Self { cost }
    }

    pub fn hash(&self, password: &str) -> Result<String, AppError> {
        hash(password, self.cost).map_err(|e| AppError::Internal(anyhow::anyhow!(e)))
    }

    pub fn verify(&self, password: &str, hash: &str) -> Result<bool, AppError> {
        verify(password, hash).map_err(|e| AppError::Internal(anyhow::anyhow!(e)))
    }
}

impl Default for HashService {
    fn default() -> Self {
        Self::new(DEFAULT_COST)
    }
}
