use tonic::{Request, Response, Status};

use crate::services::{
    auth::{AuthService, LoginParams, RefreshTokenParams},
    jwt::TokenContext,
};

use shared::proto::auth::{
    GameLoginRequest, GameLoginResponse, RefreshGameTokenRequest, RefreshGameTokenResponse,
    auth_service_server::AuthService as GrpcAuthService,
};

pub struct GrpcAuthServiceImpl {
    auth_service: AuthService,
}

impl GrpcAuthServiceImpl {
    pub fn new(auth_service: AuthService) -> Self {
        Self { auth_service }
    }
}

#[tonic::async_trait]
impl GrpcAuthService for GrpcAuthServiceImpl {
    async fn game_login(
        &self,
        request: Request<GameLoginRequest>,
    ) -> Result<Response<GameLoginResponse>, Status> {
        let req = request.into_inner();

        let params = LoginParams {
            email: req.email,
            password: req.password,
            context: TokenContext::Game,
        };

        let result = self.auth_service.login(params).await?;

        Ok(Response::new(GameLoginResponse {
            access_token: result.access_token,
            refresh_token: result.refresh_token,
        }))
    }

    async fn refresh_game_token(
        &self,
        request: Request<RefreshGameTokenRequest>,
    ) -> Result<Response<RefreshGameTokenResponse>, Status> {
        let req = request.into_inner();

        let params = RefreshTokenParams {
            refresh_token: req.refresh_token,
            context: TokenContext::Game,
        };

        let result = self.auth_service.refresh_token(params).await?;

        Ok(Response::new(RefreshGameTokenResponse {
            access_token: result.access_token,
            refresh_token: result.refresh_token,
        }))
    }
}
