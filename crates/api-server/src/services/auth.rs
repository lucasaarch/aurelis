use crate::models::account::AccountModel;
use chrono::Utc;

use crate::error::AppError;
use crate::repositories::account::{CreateAccountParams, PgAccountRepository};
use crate::services::hash::HashService;
use crate::services::jwt::{JwtService, TokenContext};

pub struct RegisterParams {
    pub email: String,
    pub password: String,
}

pub struct LoginParams {
    pub email: String,
    pub password: String,
    pub context: TokenContext,
}

pub struct RefreshTokenParams {
    pub refresh_token: String,
    pub context: TokenContext,
}

pub struct LoginResult {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Clone)]
pub struct AuthService {
    repository: PgAccountRepository,
    hash_service: HashService,
    jwt_service: JwtService,
}

impl AuthService {
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

    pub async fn authenticate(
        &self,
        token: &str,
        context: TokenContext,
    ) -> Result<AccountModel, AppError> {
        let claims = self.jwt_service.verify_with_context(token, context)?;

        self.repository
            .find_by_id(claims.sub)
            .await
            .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))
    }

    pub async fn register(&self, params: RegisterParams) -> Result<AccountModel, AppError> {
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
        let account = self
            .repository
            .find_by_email_with_hash(&params.email)
            .await
            .map_err(|_| AppError::Unauthorized("Email or password is incorrect".to_string()))?;

        let valid = self
            .hash_service
            .verify(&params.password, &account.password_hash)?;

        if !valid {
            return Err(AppError::Unauthorized(
                "Email or password is incorrect".to_string(),
            ));
        }

        match params.context {
            TokenContext::Game => self.check_if_can_perform_game_login(&account)?,
            TokenContext::Web => (),
        }

        let access_token = self
            .jwt_service
            .sign_with_context(account.id, params.context)?;

        let refresh_token = self
            .jwt_service
            .sign_refresh_with_context(account.id, params.context)?;

        Ok(LoginResult {
            access_token,
            refresh_token,
        })
    }

    pub async fn refresh_token(&self, params: RefreshTokenParams) -> Result<LoginResult, AppError> {
        let claims = self
            .jwt_service
            .verify_refresh_with_context(&params.refresh_token, params.context)?;

        let account = self
            .repository
            .find_by_id(claims.sub)
            .await
            .map_err(|_| AppError::Unauthorized("Invalid refresh token".to_string()))?;

        match params.context {
            TokenContext::Game => self.check_if_can_perform_game_login(&account)?,
            TokenContext::Web => (),
        }

        let access_token = self
            .jwt_service
            .sign_with_context(account.id, params.context)?;

        let refresh_token = self
            .jwt_service
            .sign_refresh_with_context(account.id, params.context)?;

        Ok(LoginResult {
            access_token,
            refresh_token,
        })
    }

    fn check_if_can_perform_game_login(&self, account: &AccountModel) -> Result<(), AppError> {
        if account.banned_at.is_some() {
            let reason = account
                .banned_reason
                .as_deref()
                .unwrap_or("No reason provided");

            return Err(AppError::PermissionDenied(format!(
                "Account permanently banned: {reason}"
            )));
        }

        if let Some(until) = account.suspended_until {
            let now = Utc::now().naive_utc();
            if until > now {
                let secs = (until - now).num_seconds();
                let days = secs / 86_400;
                let hours = (secs % 86_400) / 3_600;
                let minutes = (secs % 3_600) / 60;

                return Err(AppError::PermissionDenied(format!(
                    "Account suspended for {days}d {hours}h {minutes}m"
                )));
            }
        }

        Ok(())
    }
}
