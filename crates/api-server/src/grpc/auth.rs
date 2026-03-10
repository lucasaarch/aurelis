use tonic::{Request, Response, Status};

use crate::{
    proto::auth::{GameLoginRequest, GameLoginResponse, RefreshGameTokenRequest, auth_service_server::AuthService as GrpcAuthService},
    services::{auth::{AuthService, LoginParams, RefreshTokenParams}, jwt::TokenContext},
};

pub struct GrpcAuthServiceImpl(pub AuthService);

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

        let result = self
            .0
            .login(params)
            .await
            .map_err(|err| Status::unauthenticated(err.to_string()))?;

        Ok(Response::new(GameLoginResponse {
            access_token: result.access_token,
            refresh_token: result.refresh_token,
        }))
    }

    async fn refresh_game_token(
        &self,
        request: Request<RefreshGameTokenRequest>,
    ) -> Result<Response<GameLoginResponse>, Status> {
        let req = request.into_inner();

        let params = RefreshTokenParams {
            refresh_token: req.refresh_token,
            context: TokenContext::Game,
        };

        let result = self
            .0
            .refresh_token(params)
            .await
            .map_err(|err| Status::unauthenticated(err.to_string()))?;

        Ok(Response::new(GameLoginResponse {
            access_token: result.access_token,
            refresh_token: result.refresh_token,
        }))
    }
}
