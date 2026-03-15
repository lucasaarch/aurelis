use std::sync::Arc;

use tonic::transport::Server;
use tonic_reflection::server::Builder as ReflectionBuilder;
use tracing::info;

use crate::app::state::AppState;
use crate::grpc::auth::GrpcAuthServiceImpl;
use crate::grpc::character::GrpcCharacterServiceImpl;
use crate::grpc::internal_game::GrpcInternalGameServiceImpl;
use crate::grpc::inventory::GrpcInventoryServiceImpl;
use shared::proto::auth::auth_service_server::AuthServiceServer;
use shared::proto::character::character_service_server::CharacterServiceServer;
use shared::proto::internal_game::internal_game_service_server::InternalGameServiceServer;
use shared::proto::inventory::inventory_service_server::InventoryServiceServer;

pub async fn serve_grpc(state: Arc<AppState>, addr: &str) {
    info!("gRPC server listening on {addr}");

    let grpc_auth_service = GrpcAuthServiceImpl::new(state.auth_service.clone());
    let grpc_character_service =
        GrpcCharacterServiceImpl::new(state.auth_service.clone(), state.character_service.clone());
    let grpc_inventory_service = GrpcInventoryServiceImpl::new(
        state.auth_service.clone(),
        state.inventory_service.clone(),
        state.character_service.clone(),
    );
    let grpc_internal_game_service = GrpcInternalGameServiceImpl::new(
        state.character_service.clone(),
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
        .add_service(AuthServiceServer::new(grpc_auth_service))
        .add_service(CharacterServiceServer::new(grpc_character_service))
        .add_service(InventoryServiceServer::new(grpc_inventory_service))
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
