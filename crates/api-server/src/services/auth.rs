use shared::models::account::Account;
use tonic::{Request, Response, Status};

use crate::error::AppError;
use crate::proto::auth::auth_service_server::AuthService as GrpcAuthService;
use crate::proto::auth::{GameLoginRequest, GameLoginResponse};
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

        // For now create a refresh token as well. Using the same signing method
        // until a separate refresh-token implementation exists.
        let refresh_token = self
            .jwt_service
            .sign_with_context(account.id, TokenContext::Web)?;

        Ok(LoginResult {
            access_token,
            refresh_token,
        })
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

        let access_token = self
            .jwt_service
            .sign_with_context(account.id, TokenContext::Game)
            .map_err(|_| Status::internal("Failed to generate token"))?;

        // Produce a refresh token as well (placeholder: same signing for now).
        let refresh_token = self
            .jwt_service
            .sign_with_context(account.id, TokenContext::Game)
            .map_err(|_| Status::internal("Failed to generate token"))?;

        Ok(Response::new(GameLoginResponse {
            access_token,
            refresh_token,
        }))
    }
}
