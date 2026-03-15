use shared::data::cities::find_item_by_slug;

use crate::resources::internal_api::{InternalApi, PlayableCharacterSnapshot};

pub trait GemSocketApi {
    fn socket_gem(
        &self,
        account_id: uuid::Uuid,
        character_id: uuid::Uuid,
        equipment_slot: String,
        inventory_type: String,
        slot: i16,
        socket_index: i16,
    ) -> Result<(), String>;

    fn load_playable_character(
        &self,
        account_id: uuid::Uuid,
        character_id: uuid::Uuid,
    ) -> Result<PlayableCharacterSnapshot, String>;
}

impl GemSocketApi for InternalApi {
    fn socket_gem(
        &self,
        account_id: uuid::Uuid,
        character_id: uuid::Uuid,
        equipment_slot: String,
        inventory_type: String,
        slot: i16,
        socket_index: i16,
    ) -> Result<(), String> {
        InternalApi::socket_gem(
            self,
            account_id,
            character_id,
            equipment_slot,
            inventory_type,
            slot,
            socket_index,
        )
    }

    fn load_playable_character(
        &self,
        account_id: uuid::Uuid,
        character_id: uuid::Uuid,
    ) -> Result<PlayableCharacterSnapshot, String> {
        InternalApi::load_playable_character(self, account_id, character_id)
    }
}

pub fn socket_gem(
    internal_api: &impl GemSocketApi,
    snapshot: &PlayableCharacterSnapshot,
    equipment_slot: &str,
    inventory_type: &str,
    slot: i16,
    socket_index: i16,
) -> Result<PlayableCharacterSnapshot, String> {
    let equipped = snapshot
        .equipment
        .iter()
        .find(|entry| entry.slot == equipment_slot)
        .ok_or_else(|| format!("equipment slot '{}' is empty", equipment_slot))?;
    let equipment_instance = snapshot
        .item_instances
        .iter()
        .find(|instance| instance.id == equipped.item_instance_id)
        .ok_or_else(|| {
            format!(
                "missing equipped item instance '{}'",
                equipped.item_instance_id
            )
        })?;
    let equipment_item = find_item_by_slug(&equipment_instance.item_slug)
        .ok_or_else(|| format!("unknown item slug '{}'", equipment_instance.item_slug))?;
    let slot_config = equipment_item
        .kind
        .gem_slots()
        .ok_or_else(|| format!("item '{}' does not support gem slots", equipment_item.slug))?;
    let total_slots = slot_config.base_slots
        + equipment_instance
            .bonus_gem_slots
            .clamp(0, slot_config.max_bonus_slots);
    if socket_index < 0 || socket_index >= total_slots {
        return Err("invalid gem socket index".to_string());
    }
    let inventory = snapshot
        .inventories
        .iter()
        .find(|inventory| inventory.inventory_type == inventory_type)
        .ok_or_else(|| format!("inventory '{}' does not exist", inventory_type))?;
    let inventory_item = inventory
        .items
        .iter()
        .find(|item| item.slot_index == slot)
        .ok_or_else(|| "inventory slot is empty".to_string())?;
    let gem_instance_id = inventory_item
        .item_instance_id
        .ok_or_else(|| "inventory slot does not hold an item instance".to_string())?;
    let gem_instance = snapshot
        .item_instances
        .iter()
        .find(|instance| instance.id == gem_instance_id)
        .ok_or_else(|| format!("missing gem item instance '{}'", gem_instance_id))?;
    let gem_item = find_item_by_slug(&gem_instance.item_slug)
        .ok_or_else(|| format!("unknown item slug '{}'", gem_instance.item_slug))?;
    if gem_item.kind.gem_modifiers().is_none() && gem_item.kind.gem_effect_rolls().is_none() {
        return Err(format!("item '{}' is not a gem", gem_item.slug));
    }

    internal_api.socket_gem(
        snapshot.account_id,
        snapshot.character_id,
        equipment_slot.to_string(),
        inventory_type.to_string(),
        slot,
        socket_index,
    )?;

    internal_api.load_playable_character(snapshot.account_id, snapshot.character_id)
}
