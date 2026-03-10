use shared::models::account::Account;

use crate::error::AppError;
use crate::repositories::account::{CreateAccountParams, PgAccountRepository};
use crate::services::hash::HashService;

pub struct RegisterParams {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone)]
pub struct AccountService {
    repository: PgAccountRepository,
    hash_service: HashService,
}

impl AccountService {
    pub fn new(repository: PgAccountRepository, hash_service: HashService) -> Self {
        Self {
            repository,
            hash_service,
        }
    }

    pub async fn register(&self, params: RegisterParams) -> Result<Account, AppError> {
        let password_hash = self.hash_service.hash(&params.password)?;

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
