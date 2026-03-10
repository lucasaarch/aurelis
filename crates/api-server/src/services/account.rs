use shared::models::account::Account;

use crate::error::AppError;
use crate::repositories::account::{CreateAccountParams, PgAccountRepository};
use crate::services::password::PasswordHasher;

pub struct RegisterParams {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone)]
pub struct AccountService {
    repository: PgAccountRepository,
    password_hasher: PasswordHasher,
}

impl AccountService {
    pub fn new(repository: PgAccountRepository, password_hasher: PasswordHasher) -> Self {
        Self {
            repository,
            password_hasher,
        }
    }

    pub async fn register(&self, params: RegisterParams) -> Result<Account, AppError> {
        if let Ok(_) = self.repository.find_by_email(&params.email).await {
            return Err(AppError::Conflict("Email already exists".to_string()));
        }

        if let Ok(_) = self.repository.find_by_username(&params.username).await {
            return Err(AppError::Conflict("Username already exists".to_string()));
        }
        
        let password_hash = self.password_hasher.hash(&params.password)?;

        self.repository
            .create(CreateAccountParams {
                username: params.username,
                email: params.email,
                password_hash,
            })
            .await
            .map_err(Into::into)
    }
}
