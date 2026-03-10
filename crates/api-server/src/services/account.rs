use shared::models::account::Account;

use crate::error::AppError;
use crate::repositories::account::{CreateAccountParams, PgAccountRepository};
use crate::services::hash::HashService;
use crate::services::jwt::JwtService;

pub struct RegisterParams {
    pub email: String,
    pub password: String,
}

pub struct LoginParams {
    pub email: String,
    pub password: String,
}

pub struct LoginResult {
    pub token: String,
}

#[derive(Clone)]
pub struct AccountService {
    repository: PgAccountRepository,
    hash_service: HashService,
    jwt_service: JwtService,
}

impl AccountService {
    pub fn new(
        repository: PgAccountRepository,
        hash_service: HashService,
        jwt_service: JwtService,
    ) -> Self {
        Self {
            repository,
            hash_service,
            jwt_service,
        }
    }

    pub async fn register(&self, params: RegisterParams) -> Result<Account, AppError> {
        let password_hash = self.hash_service.hash(&params.password)?;

        self.repository
            .create(CreateAccountParams {
                email: params.email,
                password_hash,
            })
            .await
            .map_err(Into::into)
    }

    pub async fn login(&self, params: LoginParams) -> Result<LoginResult, AppError> {
        let (account, password_hash) = self
            .repository
            .find_by_email_with_hash(&params.email)
            .await
            .map_err(|_| AppError::Unauthorized)?;

        let valid = self.hash_service.verify(&params.password, &password_hash)?;

        if !valid {
            return Err(AppError::Unauthorized);
        }

        let token = self.jwt_service.sign(account.id)?;

        Ok(LoginResult { token })
    }
}
