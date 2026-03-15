use shared::data::cities::find_item_by_slug;
use shared::models::item_data::ItemKind;

use crate::resources::internal_api::{InternalApi, PlayableCharacterSnapshot};

pub trait CharacterSnapshotApi {
    fn equip_inventory_item(
        &self,
        account_id: uuid::Uuid,
        character_id: uuid::Uuid,
        inventory_type: String,
        slot: i16,
    ) -> Result<(), String>;

    fn unequip_item(
        &self,
        account_id: uuid::Uuid,
        character_id: uuid::Uuid,
        equipment_slot: String,
    ) -> Result<(), String>;

    fn load_playable_character(
        &self,
        account_id: uuid::Uuid,
        character_id: uuid::Uuid,
    ) -> Result<PlayableCharacterSnapshot, String>;
}

impl CharacterSnapshotApi for InternalApi {
    fn equip_inventory_item(
        &self,
        account_id: uuid::Uuid,
        character_id: uuid::Uuid,
        inventory_type: String,
        slot: i16,
    ) -> Result<(), String> {
        InternalApi::equip_inventory_item(self, account_id, character_id, inventory_type, slot)
    }

    fn unequip_item(
        &self,
        account_id: uuid::Uuid,
        character_id: uuid::Uuid,
        equipment_slot: String,
    ) -> Result<(), String> {
        InternalApi::unequip_item(self, account_id, character_id, equipment_slot)
    }

    fn load_playable_character(
        &self,
        account_id: uuid::Uuid,
        character_id: uuid::Uuid,
    ) -> Result<PlayableCharacterSnapshot, String> {
        InternalApi::load_playable_character(self, account_id, character_id)
    }
}

pub fn equip_item(
    internal_api: &impl CharacterSnapshotApi,
    snapshot: &PlayableCharacterSnapshot,
    inventory_type: &str,
    slot: i16,
) -> Result<PlayableCharacterSnapshot, String> {
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

    let item_instance_id = inventory_item
        .item_instance_id
        .ok_or_else(|| "inventory slot does not hold an item instance".to_string())?;
    let item_instance = snapshot
        .item_instances
        .iter()
        .find(|item| item.id == item_instance_id)
        .ok_or_else(|| format!("missing item instance '{}'", item_instance_id))?;
    let item_data = find_item_by_slug(&item_instance.item_slug)
        .ok_or_else(|| format!("unknown item slug '{}'", item_instance.item_slug))?;
    if item_data.kind.equipment_slot().is_none() {
        return Err(format!("item '{}' is not equippable", item_data.slug));
    }
    match &item_data.kind {
        ItemKind::Weapon(data) => {
            if snapshot.level < data.level_req {
                return Err(format!(
                    "character level {} is below required level {}",
                    snapshot.level, data.level_req
                ));
            }
            if let Some(restriction) = data.class {
                if snapshot.base_character_slug != restriction
                    && snapshot.current_class_slug != restriction
                {
                    return Err(format!(
                        "item '{}' is restricted to '{}'",
                        item_data.slug, restriction
                    ));
                }
            }
        }
        ItemKind::Armor(data) => {
            if snapshot.level < data.level_req {
                return Err(format!(
                    "character level {} is below required level {}",
                    snapshot.level, data.level_req
                ));
            }
        }
        _ => {}
    }

    internal_api.equip_inventory_item(
        snapshot.account_id,
        snapshot.character_id,
        inventory_type.to_string(),
        slot,
    )?;

    internal_api.load_playable_character(snapshot.account_id, snapshot.character_id)
}

pub fn unequip_item(
    internal_api: &impl CharacterSnapshotApi,
    snapshot: &PlayableCharacterSnapshot,
    equipment_slot: &str,
) -> Result<PlayableCharacterSnapshot, String> {
    let equipped = snapshot
        .equipment
        .iter()
        .find(|item| item.slot == equipment_slot)
        .ok_or_else(|| format!("equipment slot '{}' is empty", equipment_slot))?;

    let item_instance = snapshot
        .item_instances
        .iter()
        .find(|item| item.id == equipped.item_instance_id)
        .ok_or_else(|| {
            format!(
                "missing equipped item instance '{}'",
                equipped.item_instance_id
            )
        })?;
    let item_data = find_item_by_slug(&item_instance.item_slug)
        .ok_or_else(|| format!("unknown item slug '{}'", item_instance.item_slug))?;
    if item_data.kind.equipment_slot().is_none() {
        return Err(format!("item '{}' is not equippable", item_data.slug));
    }

    internal_api.unequip_item(
        snapshot.account_id,
        snapshot.character_id,
        equipment_slot.to_string(),
    )?;

    internal_api.load_playable_character(snapshot.account_id, snapshot.character_id)
}
