use chrono::Utc;
use shared::models::account::Account;
use tonic::{Request, Response, Status};

use crate::error::AppError;
use crate::proto::auth::auth_service_server::AuthService as GrpcAuthService;
use crate::proto::auth::{GameLoginRequest, GameLoginResponse, RefreshGameTokenRequest};
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

    /// Web-specific login: always issues a token signed for the Web context.
    pub async fn web_login(&self, params: LoginParams) -> Result<LoginResult, AppError> {
        let (account, password_hash) = self
            .repository
            .find_by_email_with_hash(&params.email)
            .await
            .map_err(|_| AppError::Unauthorized)?;

        let valid = self.hash_service.verify(&params.password, &password_hash)?;

        if !valid {
            return Err(AppError::Unauthorized);
        }

        let access_token = self
            .jwt_service
            .sign_with_context(account.id, TokenContext::Web)?;

        // Create a refresh token using the refresh signing method.
        let refresh_token = self
            .jwt_service
            .sign_refresh_with_context(account.id, TokenContext::Web)?;

        Ok(LoginResult {
            access_token,
            refresh_token,
        })
    }

    /// Refresh a web refresh token, returning a new access and refresh token pair.
    pub async fn refresh_web_token(&self, refresh_token: &str) -> Result<LoginResult, AppError> {
        // Verify refresh token using the Web context
        let claims = self
            .jwt_service
            .verify_refresh_with_context(refresh_token, TokenContext::Web)?;

        // Ensure account still exists
        let account = self
            .repository
            .find_by_id(claims.sub)
            .await
            .map_err(|_| AppError::Unauthorized)?;

        // Issue new tokens
        let access_token = self
            .jwt_service
            .sign_with_context(account.id, TokenContext::Web)?;

        let refresh_token = self
            .jwt_service
            .sign_refresh_with_context(account.id, TokenContext::Web)?;

        Ok(LoginResult {
            access_token,
            refresh_token,
        })
    }

    fn check_game_login(&self, account: &Account) -> Result<(), Status> {
        // Check permanent ban
        if account.banned_at.is_some() {
            let reason = account
                .banned_reason
                .as_ref()
                .map(|s| s.as_str())
                .unwrap_or("No reason provided");

            return Err(Status::permission_denied(format!(
                "Account banned: {}",
                reason
            )));
        }

        // Check suspension
        if let Some(until) = account.suspended_until {
            let now = Utc::now().naive_utc();
            if until > now {
                let secs = (until - now).num_seconds();
                let days = secs / 86_400;
                let hours = (secs % 86_400) / 3_600;
                let minutes = (secs % 3_600) / 60;

                return Err(Status::permission_denied(format!(
                    "Account suspended for {} days, {} hours and {} minutes",
                    days, hours, minutes
                )));
            }
        }

        Ok(())
    }
}

#[tonic::async_trait]
impl GrpcAuthService for AuthService {
    async fn game_login(
        &self,
        request: Request<GameLoginRequest>,
    ) -> Result<Response<GameLoginResponse>, Status> {
        let req = request.into_inner();

        let (account, password_hash) = self
            .repository
            .find_by_email_with_hash(&req.email)
            .await
            .map_err(|_| Status::unauthenticated("Invalid email or password"))?;

        let valid = self
            .hash_service
            .verify(&req.password, &password_hash)
            .map_err(|_| Status::unauthenticated("Invalid email or password"))?;

        if !valid {
            return Err(Status::unauthenticated("Invalid email or password"));
        }

        // Verify account status (ban/suspension)
        self.check_game_login(&account)?;

        let access_token = self
            .jwt_service
            .sign_with_context(account.id, TokenContext::Game)
            .map_err(|_| Status::internal("Failed to generate token"))?;

        // Produce a refresh token as well using refresh signing.
        let refresh_token = self
            .jwt_service
            .sign_refresh_with_context(account.id, TokenContext::Game)
            .map_err(|_| Status::internal("Failed to generate token"))?;

        Ok(Response::new(GameLoginResponse {
            access_token,
            refresh_token,
        }))
    }

    async fn refresh_game_token(
        &self,
        request: Request<RefreshGameTokenRequest>,
    ) -> Result<Response<GameLoginResponse>, Status> {
        let req = request.into_inner();

        // Verify refresh token using the Game context
        let claims = self
            .jwt_service
            .verify_refresh_with_context(&req.refresh_token, TokenContext::Game)
            .map_err(|_| Status::unauthenticated("Invalid or expired refresh token"))?;

        // Ensure account still exists
        let account = self
            .repository
            .find_by_id(claims.sub)
            .await
            .map_err(|_| Status::unauthenticated("Account not found"))?;

        // Verify account status (ban/suspension)
        self.check_game_login(&account)?;

        // Issue new tokens
        let access_token = self
            .jwt_service
            .sign_with_context(account.id, TokenContext::Game)
            .map_err(|_| Status::internal("Failed to generate access token"))?;

        let refresh_token = self
            .jwt_service
            .sign_refresh_with_context(account.id, TokenContext::Game)
            .map_err(|_| Status::internal("Failed to generate refresh token"))?;

        Ok(Response::new(GameLoginResponse {
            access_token,
            refresh_token,
        }))
    }
}
