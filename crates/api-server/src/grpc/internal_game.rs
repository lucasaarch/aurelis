use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::services::{character::CharacterService, item_instance::ItemInstanceService};

use shared::proto::internal_game::{
    LoadPlayableCharacterRequest, LoadPlayableCharacterResponse, PersistItemInstanceStateRequest,
    PersistItemInstanceStateResponse, PersistedEquipmentSnapshot, PersistedInventoryItemSnapshot,
    PersistedInventorySnapshot, PersistedItemInstanceGemSnapshot, PersistedItemInstanceSnapshot,
    internal_game_service_server::InternalGameService,
};

pub struct GrpcInternalGameServiceImpl {
    character_service: CharacterService,
    item_instance_service: ItemInstanceService,
}

impl GrpcInternalGameServiceImpl {
    pub fn new(
        character_service: CharacterService,
        item_instance_service: ItemInstanceService,
    ) -> Self {
        Self {
            character_service,
            item_instance_service,
        }
    }
}

#[tonic::async_trait]
impl InternalGameService for GrpcInternalGameServiceImpl {
    async fn load_playable_character(
        &self,
        request: Request<LoadPlayableCharacterRequest>,
    ) -> Result<Response<LoadPlayableCharacterResponse>, Status> {
        let req = request.into_inner();
        let account_id = Uuid::parse_str(&req.account_id)
            .map_err(|_| Status::invalid_argument("invalid account_id"))?;
        let character_id = Uuid::parse_str(&req.character_id)
            .map_err(|_| Status::invalid_argument("invalid character_id"))?;

        let result = self
            .character_service
            .load_playable_character(account_id, character_id)
            .await?;

        Ok(Response::new(LoadPlayableCharacterResponse {
            character_id: result.character_id.to_string(),
            name: result.name,
            base_character_slug: result.base_character_slug,
            current_class_slug: result.current_class_slug,
            level: result.level as i32,
            account_id: result.account_id.to_string(),
            experience: result.experience,
            credits: result.credits,
            beginner_skill_unlocked: result.skill_unlocks.beginner,
            intermediate_skill_unlocked: result.skill_unlocks.intermediate,
            inventories: result
                .inventories
                .into_iter()
                .map(|inventory| PersistedInventorySnapshot {
                    id: inventory.id.to_string(),
                    inventory_type: inventory.inventory_type,
                    capacity: inventory.capacity as i32,
                    items: inventory
                        .items
                        .into_iter()
                        .map(|item| PersistedInventoryItemSnapshot {
                            id: item.id.to_string(),
                            inventory_id: item.inventory_id.to_string(),
                            inventory_type: item.inventory_type,
                            slot_index: item.slot_index as i32,
                            quantity: item.quantity as i32,
                            item_instance_id: item.item_instance_id.map(|value| value.to_string()),
                            item_id: item.item_id.map(|value| value.to_string()),
                            item_slug: item.item_slug,
                        })
                        .collect(),
                })
                .collect(),
            equipment: result
                .equipment
                .into_iter()
                .map(|equipment| PersistedEquipmentSnapshot {
                    slot: equipment.slot,
                    item_instance_id: equipment.item_instance_id.to_string(),
                })
                .collect(),
            item_instances: result
                .item_instances
                .into_iter()
                .map(|item_instance| PersistedItemInstanceSnapshot {
                    id: item_instance.id.to_string(),
                    item_id: item_instance.item_id.to_string(),
                    item_slug: item_instance.item_slug,
                    inventory_type: item_instance.inventory_type,
                    refinement: item_instance.refinement as i32,
                    bonus_gem_slots: item_instance.bonus_gem_slots as i32,
                    attributes_json: item_instance.attributes_json,
                    in_shared_storage: item_instance.in_shared_storage,
                    in_trade: item_instance.in_trade,
                    gems: item_instance
                        .gems
                        .into_iter()
                        .map(|gem| PersistedItemInstanceGemSnapshot {
                            slot_index: gem.slot_index as i32,
                            gem_instance_id: gem.gem_instance_id.to_string(),
                        })
                        .collect(),
                })
                .collect(),
        }))
    }

    async fn persist_item_instance_state(
        &self,
        request: Request<PersistItemInstanceStateRequest>,
    ) -> Result<Response<PersistItemInstanceStateResponse>, Status> {
        let req = request.into_inner();
        let account_id = Uuid::parse_str(&req.account_id)
            .map_err(|_| Status::invalid_argument("invalid account_id"))?;
        let character_id = Uuid::parse_str(&req.character_id)
            .map_err(|_| Status::invalid_argument("invalid character_id"))?;
        let item_instance_id = Uuid::parse_str(&req.item_instance_id)
            .map_err(|_| Status::invalid_argument("invalid item_instance_id"))?;

        self.item_instance_service
            .persist_state(
                account_id,
                character_id,
                item_instance_id,
                req.bonus_gem_slots as i16,
                req.attributes_json,
            )
            .await?;

        Ok(Response::new(PersistItemInstanceStateResponse {}))
    }
}
