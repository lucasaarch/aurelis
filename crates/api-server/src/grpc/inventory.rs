use tonic::{Request, Response, Status};

use shared::proto::inventory::{
    DeleteItemRequest, DeleteItemResponse, InventoryItem as ProtoInventoryItem, ListItemsRequest,
    ListItemsResponse, MoveItemRequest, MoveItemResponse,
};

use crate::{
    services::{
        auth::AuthService, character::CharacterService, inventory::InventoryService,
        jwt::TokenContext,
    },
    utils::{
        datetime::format_naive_datetime, extractors::extract_access_token_from_metadata,
        parsers::parse_uuid,
    },
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
impl shared::proto::inventory::inventory_service_server::InventoryService
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
                Status::from(e)
            })?;

        Ok(Response::new(MoveItemResponse {}))
    }

    async fn delete_item(
        &self,
        request: Request<DeleteItemRequest>,
    ) -> Result<Response<DeleteItemResponse>, Status> {
        let token = extract_access_token_from_metadata(request.metadata())?;
        let req = request.into_inner();

        let account = self
            .auth_service
            .authenticate(&token, TokenContext::Game)
            .await?;

        let character_id = parse_uuid(&req.character_id)?;

        self.character_service
            .verify_ownership(account.id, character_id)
            .await?;

        self.inventory_service
            .delete_slot(character_id, req.inventory_type, req.slot as i16)
            .await
            .map_err(|e| {
                tracing::error!("Failed to delete item: {:?}", e);
                Status::from(e)
            })?;

        Ok(Response::new(DeleteItemResponse {}))
    }

    async fn list_items(
        &self,
        request: Request<ListItemsRequest>,
    ) -> Result<Response<ListItemsResponse>, Status> {
        let token = extract_access_token_from_metadata(request.metadata())?;
        let req = request.into_inner();

        let account = self
            .auth_service
            .authenticate(&token, TokenContext::Game)
            .await?;

        let character_id = parse_uuid(&req.character_id)?;

        // ensure the authenticated account owns the character being listed
        self.character_service
            .verify_ownership(account.id, character_id)
            .await?;

        let items = self
            .inventory_service
            .list_items(character_id, req.inventory_type)
            .await
            .map_err(|e| {
                tracing::error!("Failed to list items: {:?}", e);
                Status::from(e)
            })?;

        let proto_items: Vec<ProtoInventoryItem> = items
            .into_iter()
            .map(|detailed| {
                // `detailed` is InventoryDetailedItem containing both inventory and item fields
                let proto_detail = shared::proto::inventory::ItemDetail {
                    id: detailed.item_id.map(|u| u.to_string()).unwrap_or_default(),
                    name: detailed.name.clone(),
                    description: detailed.description.clone().unwrap_or_default(),
                    icon: String::new(),
                    rarity: detailed.rarity.to_string(),
                    max_stack: detailed.max_stack as i32,
                    base_stats: std::collections::HashMap::new(),
                    attributes: vec![],
                };

                ProtoInventoryItem {
                    id: detailed.id.to_string(),
                    inventory_id: detailed.inventory_id.to_string(),
                    item_instance_id: detailed.item_instance_id.map(|u| u.to_string()),
                    item_id: detailed.item_id.map(|u| u.to_string()),
                    slot_index: detailed.slot_index as i32,
                    quantity: detailed.quantity as i32,
                    acquired_at: format_naive_datetime(&detailed.acquired_at),
                    // item is always present for detailed rows
                    item: Some(proto_detail),
                }
            })
            .collect();

        Ok(Response::new(ListItemsResponse { items: proto_items }))
    }
}
