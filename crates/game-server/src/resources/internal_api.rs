use bevy::prelude::Resource;
use tokio::runtime::Runtime;
use tonic::metadata::MetadataValue;
use tonic::service::Interceptor;
use tonic::transport::Endpoint;
use tonic::{Request, Status};
use uuid::Uuid;

use shared::models::character_data::CharacterSkillUnlocks;
use shared::proto::internal_game::{
    LoadPlayableCharacterRequest, LoadPlayableCharacterResponse, PersistItemInstanceStateRequest,
    PersistedEquipmentSnapshot, PersistedInventoryItemSnapshot, PersistedInventorySnapshot,
    PersistedItemInstanceGemSnapshot, PersistedItemInstanceSnapshot,
    internal_game_service_client::InternalGameServiceClient,
};

#[derive(Debug, Resource)]
pub struct InternalApi {
    grpc_addr: String,
    bearer_token: String,
    runtime: Runtime,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PlayableCharacterSnapshot {
    pub account_id: Uuid,
    pub character_id: Uuid,
    pub name: String,
    pub base_character_slug: String,
    pub current_class_slug: String,
    pub level: i16,
    pub experience: i64,
    pub credits: i64,
    pub skill_unlocks: CharacterSkillUnlocks,
    pub inventories: Vec<PersistedInventory>,
    pub equipment: Vec<PersistedEquipment>,
    pub item_instances: Vec<PersistedItemInstance>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PersistedInventory {
    pub id: Uuid,
    pub inventory_type: String,
    pub capacity: i16,
    pub items: Vec<PersistedInventoryItem>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PersistedInventoryItem {
    pub id: Uuid,
    pub inventory_id: Uuid,
    pub inventory_type: String,
    pub slot_index: i16,
    pub quantity: i16,
    pub item_instance_id: Option<Uuid>,
    pub item_id: Option<Uuid>,
    pub item_slug: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PersistedEquipment {
    pub slot: String,
    pub item_instance_id: Uuid,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PersistedItemInstance {
    pub id: Uuid,
    pub item_id: Uuid,
    pub item_slug: String,
    pub inventory_type: String,
    pub refinement: i16,
    pub bonus_gem_slots: i16,
    pub attributes_json: String,
    pub in_shared_storage: bool,
    pub in_trade: bool,
    pub gems: Vec<PersistedItemInstanceGem>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PersistedItemInstanceGem {
    pub slot_index: i16,
    pub gem_instance_id: Uuid,
}

impl InternalApi {
    pub fn new(grpc_addr: String, bearer_token: String) -> Self {
        Self {
            grpc_addr,
            bearer_token,
            runtime: Runtime::new().expect("failed to create tokio runtime"),
        }
    }

    pub fn load_playable_character(
        &self,
        account_id: Uuid,
        character_id: Uuid,
    ) -> Result<PlayableCharacterSnapshot, String> {
        self.runtime.block_on(async {
            let endpoint =
                Endpoint::from_shared(self.grpc_addr.clone()).map_err(|err| err.to_string())?;
            let channel = endpoint.connect().await.map_err(|err| err.to_string())?;
            let interceptor = InternalServerAuthInterceptor::new(self.bearer_token.clone());
            let mut client = InternalGameServiceClient::with_interceptor(channel, interceptor);

            let response = client
                .load_playable_character(Request::new(LoadPlayableCharacterRequest {
                    account_id: account_id.to_string(),
                    character_id: character_id.to_string(),
                }))
                .await
                .map_err(|err| err.to_string())?
                .into_inner();

            map_snapshot(response)
        })
    }

    pub fn persist_item_instance_state(
        &self,
        account_id: Uuid,
        character_id: Uuid,
        item_instance_id: Uuid,
        bonus_gem_slots: i16,
        attributes_json: String,
    ) -> Result<(), String> {
        self.runtime.block_on(async {
            let endpoint =
                Endpoint::from_shared(self.grpc_addr.clone()).map_err(|err| err.to_string())?;
            let channel = endpoint.connect().await.map_err(|err| err.to_string())?;
            let interceptor = InternalServerAuthInterceptor::new(self.bearer_token.clone());
            let mut client = InternalGameServiceClient::with_interceptor(channel, interceptor);

            client
                .persist_item_instance_state(Request::new(PersistItemInstanceStateRequest {
                    account_id: account_id.to_string(),
                    character_id: character_id.to_string(),
                    item_instance_id: item_instance_id.to_string(),
                    bonus_gem_slots: i32::from(bonus_gem_slots),
                    attributes_json,
                }))
                .await
                .map_err(|err| err.to_string())?;

            Ok(())
        })
    }
}

fn map_snapshot(
    response: LoadPlayableCharacterResponse,
) -> Result<PlayableCharacterSnapshot, String> {
    Ok(PlayableCharacterSnapshot {
        account_id: Uuid::parse_str(&response.account_id).map_err(|err| err.to_string())?,
        character_id: Uuid::parse_str(&response.character_id).map_err(|err| err.to_string())?,
        name: response.name,
        base_character_slug: response.base_character_slug,
        current_class_slug: response.current_class_slug,
        level: response.level as i16,
        experience: response.experience,
        credits: response.credits,
        skill_unlocks: CharacterSkillUnlocks {
            beginner: response.beginner_skill_unlocked,
            intermediate: response.intermediate_skill_unlocked,
        },
        inventories: response
            .inventories
            .into_iter()
            .map(map_inventory)
            .collect::<Result<Vec<_>, _>>()?,
        equipment: response
            .equipment
            .into_iter()
            .map(map_equipment)
            .collect::<Result<Vec<_>, _>>()?,
        item_instances: response
            .item_instances
            .into_iter()
            .map(map_item_instance)
            .collect::<Result<Vec<_>, _>>()?,
    })
}

fn map_inventory(response: PersistedInventorySnapshot) -> Result<PersistedInventory, String> {
    Ok(PersistedInventory {
        id: Uuid::parse_str(&response.id).map_err(|err| err.to_string())?,
        inventory_type: response.inventory_type,
        capacity: response.capacity as i16,
        items: response
            .items
            .into_iter()
            .map(map_inventory_item)
            .collect::<Result<Vec<_>, _>>()?,
    })
}

fn map_inventory_item(
    response: PersistedInventoryItemSnapshot,
) -> Result<PersistedInventoryItem, String> {
    Ok(PersistedInventoryItem {
        id: Uuid::parse_str(&response.id).map_err(|err| err.to_string())?,
        inventory_id: Uuid::parse_str(&response.inventory_id).map_err(|err| err.to_string())?,
        inventory_type: response.inventory_type,
        slot_index: response.slot_index as i16,
        quantity: response.quantity as i16,
        item_instance_id: response
            .item_instance_id
            .map(|value| Uuid::parse_str(&value).map_err(|err| err.to_string()))
            .transpose()?,
        item_id: response
            .item_id
            .map(|value| Uuid::parse_str(&value).map_err(|err| err.to_string()))
            .transpose()?,
        item_slug: response.item_slug,
    })
}

fn map_equipment(response: PersistedEquipmentSnapshot) -> Result<PersistedEquipment, String> {
    Ok(PersistedEquipment {
        slot: response.slot,
        item_instance_id: Uuid::parse_str(&response.item_instance_id)
            .map_err(|err| err.to_string())?,
    })
}

fn map_item_instance(
    response: PersistedItemInstanceSnapshot,
) -> Result<PersistedItemInstance, String> {
    Ok(PersistedItemInstance {
        id: Uuid::parse_str(&response.id).map_err(|err| err.to_string())?,
        item_id: Uuid::parse_str(&response.item_id).map_err(|err| err.to_string())?,
        item_slug: response.item_slug,
        inventory_type: response.inventory_type,
        refinement: response.refinement as i16,
        bonus_gem_slots: response.bonus_gem_slots as i16,
        attributes_json: response.attributes_json,
        in_shared_storage: response.in_shared_storage,
        in_trade: response.in_trade,
        gems: response
            .gems
            .into_iter()
            .map(map_item_instance_gem)
            .collect::<Result<Vec<_>, _>>()?,
    })
}

fn map_item_instance_gem(
    response: PersistedItemInstanceGemSnapshot,
) -> Result<PersistedItemInstanceGem, String> {
    Ok(PersistedItemInstanceGem {
        slot_index: response.slot_index as i16,
        gem_instance_id: Uuid::parse_str(&response.gem_instance_id)
            .map_err(|err| err.to_string())?,
    })
}

#[derive(Clone)]
struct InternalServerAuthInterceptor {
    bearer_header: MetadataValue<tonic::metadata::Ascii>,
}

impl InternalServerAuthInterceptor {
    fn new(token: String) -> Self {
        let bearer = format!("Bearer {token}")
            .parse()
            .expect("invalid internal bearer token");
        Self {
            bearer_header: bearer,
        }
    }
}

impl Interceptor for InternalServerAuthInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        request
            .metadata_mut()
            .insert("authorization", self.bearer_header.clone());
        Ok(request)
    }
}
