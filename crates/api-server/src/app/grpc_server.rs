use std::sync::Arc;

use tonic::transport::Server;
use tonic_reflection::server::Builder as ReflectionBuilder;
use tracing::info;

use crate::app::state::AppState;
use crate::grpc::internal_game::GrpcInternalGameServiceImpl;
use shared::proto::internal_game::internal_game_service_server::InternalGameServiceServer;

pub async fn serve_grpc(state: Arc<AppState>, addr: &str) {
    info!("gRPC server listening on {addr}");

    let grpc_internal_game_service = GrpcInternalGameServiceImpl::new(
        state.auth_service.clone(),
        state.character_service.clone(),
        state.character_skill_service.clone(),
        state.equipment_service.clone(),
        state.inventory_service.clone(),
        state.item_instance_service.clone(),
    );
    let internal_server_token = state.config.server.internal_server_token.clone();

    Server::builder()
        .add_service(
            ReflectionBuilder::configure()
                .register_encoded_file_descriptor_set(shared::proto::FILE_DESCRIPTOR_SET)
                .build_v1()
                .unwrap(),
        )
        .add_service(InternalGameServiceServer::with_interceptor(
            grpc_internal_game_service,
            move |request: tonic::Request<()>| {
                validate_internal_server_request(request, &internal_server_token)
            },
        ))
        .serve(addr.parse().unwrap())
        .await
        .expect("gRPC server error");
}

fn validate_internal_server_request(
    request: tonic::Request<()>,
    expected_token: &str,
) -> Result<tonic::Request<()>, tonic::Status> {
    let auth_header = request
        .metadata()
        .get("authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| tonic::Status::unauthenticated("missing authorization metadata"))?;

    let expected_header = format!("Bearer {expected_token}");
    if auth_header != expected_header {
        return Err(tonic::Status::permission_denied(
            "invalid internal server token",
        ));
    }

    Ok(request)
}
