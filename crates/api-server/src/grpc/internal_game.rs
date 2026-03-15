use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::services::jwt::TokenContext;
use crate::services::{
    auth::{AuthService, LoginParams},
    character::CharacterService,
    character_skill::CharacterSkillService,
    equipment::EquipmentService,
    inventory::InventoryService,
    item_instance::ItemInstanceService,
};
use crate::utils::datetime::format_naive_datetime;
use shared::models::skill_data::CharacterSkillUnlockTier;

use shared::proto::internal_game::{
    CharacterSummary, ConsumeInventoryItemRequest, ConsumeInventoryItemResponse,
    CreateCharacterRequest, CreateCharacterResponse, EquipInventoryItemRequest,
    EquipInventoryItemResponse, GameLoginRequest, GameLoginResponse, ListCharactersRequest,
    ListCharactersResponse, LoadPlayableCharacterRequest, LoadPlayableCharacterResponse,
    PersistItemInstanceStateRequest, PersistItemInstanceStateResponse, PersistedEquipmentSnapshot,
    PersistedInventoryItemSnapshot, PersistedInventorySnapshot, PersistedItemInstanceGemSnapshot,
    PersistedItemInstanceSnapshot, SocketGemRequest, SocketGemResponse, UnequipItemRequest,
    UnequipItemResponse, UnlockCharacterSkillTierRequest, UnlockCharacterSkillTierResponse,
    UpdateItemInstanceRefinementRequest, UpdateItemInstanceRefinementResponse,
    internal_game_service_server::InternalGameService,
};

pub struct GrpcInternalGameServiceImpl {
    auth_service: AuthService,
    character_service: CharacterService,
    character_skill_service: CharacterSkillService,
    equipment_service: EquipmentService,
    inventory_service: InventoryService,
    item_instance_service: ItemInstanceService,
}

impl GrpcInternalGameServiceImpl {
    pub fn new(
        auth_service: AuthService,
        character_service: CharacterService,
        character_skill_service: CharacterSkillService,
        equipment_service: EquipmentService,
        inventory_service: InventoryService,
        item_instance_service: ItemInstanceService,
    ) -> Self {
        Self {
            auth_service,
            character_service,
            character_skill_service,
            equipment_service,
            inventory_service,
            item_instance_service,
        }
    }
}

#[tonic::async_trait]
impl InternalGameService for GrpcInternalGameServiceImpl {
    async fn game_login(
        &self,
        request: Request<GameLoginRequest>,
    ) -> Result<Response<GameLoginResponse>, Status> {
        let req = request.into_inner();
        let result = self
            .auth_service
            .login(LoginParams {
                email: req.email,
                password: req.password,
                context: TokenContext::Game,
            })
            .await?;
        let account = self
            .auth_service
            .authenticate(&result.access_token, TokenContext::Game)
            .await?;

        Ok(Response::new(GameLoginResponse {
            account_id: account.id.to_string(),
        }))
    }

    async fn list_characters(
        &self,
        request: Request<ListCharactersRequest>,
    ) -> Result<Response<ListCharactersResponse>, Status> {
        let req = request.into_inner();
        let account_id = Uuid::parse_str(&req.account_id)
            .map_err(|_| Status::invalid_argument("invalid account_id"))?;
        let characters = self.character_service.list_all(account_id).await?;

        Ok(Response::new(ListCharactersResponse {
            characters: characters.into_iter().map(map_character_summary).collect(),
        }))
    }

    async fn create_character(
        &self,
        request: Request<CreateCharacterRequest>,
    ) -> Result<Response<CreateCharacterResponse>, Status> {
        let req = request.into_inner();
        let account_id = Uuid::parse_str(&req.account_id)
            .map_err(|_| Status::invalid_argument("invalid account_id"))?;
        let character = self
            .character_service
            .create(
                account_id,
                crate::services::character::CreateCharacterInput {
                    name: req.name,
                    class: req.class_slug,
                },
            )
            .await?;

        Ok(Response::new(CreateCharacterResponse {
            character: Some(map_character_summary(character)),
        }))
    }

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

    async fn unlock_character_skill_tier(
        &self,
        request: Request<UnlockCharacterSkillTierRequest>,
    ) -> Result<Response<UnlockCharacterSkillTierResponse>, Status> {
        let req = request.into_inner();
        let account_id = Uuid::parse_str(&req.account_id)
            .map_err(|_| Status::invalid_argument("invalid account_id"))?;
        let character_id = Uuid::parse_str(&req.character_id)
            .map_err(|_| Status::invalid_argument("invalid character_id"))?;
        let tier = match req.tier.as_str() {
            "beginner" => CharacterSkillUnlockTier::Beginner,
            "intermediate" => CharacterSkillUnlockTier::Intermediate,
            _ => return Err(Status::invalid_argument("invalid tier")),
        };

        self.character_skill_service
            .unlock_tier(account_id, character_id, tier)
            .await?;

        Ok(Response::new(UnlockCharacterSkillTierResponse {}))
    }

    async fn consume_inventory_item(
        &self,
        request: Request<ConsumeInventoryItemRequest>,
    ) -> Result<Response<ConsumeInventoryItemResponse>, Status> {
        let req = request.into_inner();
        let account_id = Uuid::parse_str(&req.account_id)
            .map_err(|_| Status::invalid_argument("invalid account_id"))?;
        let character_id = Uuid::parse_str(&req.character_id)
            .map_err(|_| Status::invalid_argument("invalid character_id"))?;

        self.character_service
            .verify_ownership(account_id, character_id)
            .await?;

        self.inventory_service
            .consume_item_slot(
                character_id,
                req.inventory_type,
                req.slot as i16,
                req.quantity as i16,
            )
            .await?;

        Ok(Response::new(ConsumeInventoryItemResponse {}))
    }

    async fn equip_inventory_item(
        &self,
        request: Request<EquipInventoryItemRequest>,
    ) -> Result<Response<EquipInventoryItemResponse>, Status> {
        let req = request.into_inner();
        let account_id = Uuid::parse_str(&req.account_id)
            .map_err(|_| Status::invalid_argument("invalid account_id"))?;
        let character_id = Uuid::parse_str(&req.character_id)
            .map_err(|_| Status::invalid_argument("invalid character_id"))?;

        self.character_service
            .verify_ownership(account_id, character_id)
            .await?;

        self.equipment_service
            .equip_inventory_item(character_id, req.inventory_type, req.slot as i16)
            .await?;

        Ok(Response::new(EquipInventoryItemResponse {}))
    }

    async fn unequip_item(
        &self,
        request: Request<UnequipItemRequest>,
    ) -> Result<Response<UnequipItemResponse>, Status> {
        let req = request.into_inner();
        let account_id = Uuid::parse_str(&req.account_id)
            .map_err(|_| Status::invalid_argument("invalid account_id"))?;
        let character_id = Uuid::parse_str(&req.character_id)
            .map_err(|_| Status::invalid_argument("invalid character_id"))?;

        self.character_service
            .verify_ownership(account_id, character_id)
            .await?;

        self.equipment_service
            .unequip_item(character_id, req.equipment_slot)
            .await?;

        Ok(Response::new(UnequipItemResponse {}))
    }

    async fn update_item_instance_refinement(
        &self,
        request: Request<UpdateItemInstanceRefinementRequest>,
    ) -> Result<Response<UpdateItemInstanceRefinementResponse>, Status> {
        let req = request.into_inner();
        let account_id = Uuid::parse_str(&req.account_id)
            .map_err(|_| Status::invalid_argument("invalid account_id"))?;
        let character_id = Uuid::parse_str(&req.character_id)
            .map_err(|_| Status::invalid_argument("invalid character_id"))?;
        let item_instance_id = Uuid::parse_str(&req.item_instance_id)
            .map_err(|_| Status::invalid_argument("invalid item_instance_id"))?;

        self.item_instance_service
            .update_refinement(
                account_id,
                character_id,
                item_instance_id,
                req.refinement as i16,
            )
            .await?;

        Ok(Response::new(UpdateItemInstanceRefinementResponse {}))
    }

    async fn socket_gem(
        &self,
        request: Request<SocketGemRequest>,
    ) -> Result<Response<SocketGemResponse>, Status> {
        let req = request.into_inner();
        let account_id = Uuid::parse_str(&req.account_id)
            .map_err(|_| Status::invalid_argument("invalid account_id"))?;
        let character_id = Uuid::parse_str(&req.character_id)
            .map_err(|_| Status::invalid_argument("invalid character_id"))?;

        self.character_service
            .verify_ownership(account_id, character_id)
            .await?;

        self.equipment_service
            .socket_gem(
                character_id,
                req.equipment_slot,
                req.inventory_type,
                req.slot as i16,
                req.socket_index as i16,
            )
            .await?;

        Ok(Response::new(SocketGemResponse {}))
    }
}

fn map_character_summary(
    character: crate::models::player_character::PlayerCharacterModel,
) -> CharacterSummary {
    CharacterSummary {
        id: character.id.to_string(),
        name: character.name,
        level: i32::from(character.level),
        class_slug: character.current_class_slug,
        created_at: format_naive_datetime(&character.created_at),
        updated_at: format_naive_datetime(&character.updated_at),
    }
}
