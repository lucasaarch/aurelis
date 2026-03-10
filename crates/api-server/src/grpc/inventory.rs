use tonic::{Request, Response, Status};

use crate::{
    proto::inventory::{MoveItemRequest, MoveItemResponse},
    services::{
        auth::AuthService, character::CharacterService, inventory::InventoryService,
        jwt::TokenContext,
    },
    utils::{extractors::extract_access_token_from_metadata, parsers::parse_uuid},
};

pub struct GrpcInventoryServiceImpl {
    auth_service: AuthService,
    inventory_service: InventoryService,
    character_service: CharacterService,
}

impl GrpcInventoryServiceImpl {
    pub fn new(
        auth_service: AuthService,
        inventory_service: InventoryService,
        character_service: CharacterService,
    ) -> Self {
        Self {
            auth_service,
            inventory_service,
            character_service,
        }
    }
}

#[tonic::async_trait]
impl crate::proto::inventory::inventory_service_server::InventoryService
    for GrpcInventoryServiceImpl
{
    async fn move_item(
        &self,
        request: Request<MoveItemRequest>,
    ) -> Result<Response<MoveItemResponse>, Status> {
        let token = extract_access_token_from_metadata(request.metadata())?;

        let account = self
            .auth_service
            .authenticate(&token, TokenContext::Game)
            .await?;

        let req = request.into_inner();

        let character_id = parse_uuid(&req.character_id)?;

        // ensure the authenticated account owns the character being modified
        self.character_service
            .verify_ownership(account.id, character_id)
            .await?;

        self.inventory_service
            .move_item(
                character_id,
                req.inventory_type,
                req.from_slot as i16,
                req.to_slot as i16,
            )
            .await
            .map_err(|e| {
                tracing::error!("Failed to move item: {:?}", e);
                e
            })?;

        Ok(Response::new(MoveItemResponse {}))
    }
}
