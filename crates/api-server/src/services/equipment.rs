use anyhow::anyhow;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::equipment_slot::EquipmentSlotModel,
    repositories::{
        character::{PgCharacterRepository, PlayableCharacterRow},
        inventory::PgInventoryRepository,
        item::PgItemRepository,
        item_instance::PgItemInstanceRepository,
    },
};
use shared::models::{equipment_slot::EquipmentSlot, item_data::ItemData};

#[derive(Clone)]
pub struct EquipmentService {
    character_repository: PgCharacterRepository,
    inventory_repository: PgInventoryRepository,
    item_repository: PgItemRepository,
    item_instance_repository: PgItemInstanceRepository,
}

impl EquipmentService {
    pub fn new(
        character_repository: PgCharacterRepository,
        inventory_repository: PgInventoryRepository,
        item_repository: PgItemRepository,
        item_instance_repository: PgItemInstanceRepository,
    ) -> Self {
        Self {
            character_repository,
            inventory_repository,
            item_repository,
            item_instance_repository,
        }
    }

    pub async fn equip_inventory_item(
        &self,
        character_id: Uuid,
        inventory_type: String,
        slot: i16,
    ) -> Result<(), AppError> {
        let character = self
            .character_repository
            .find_playable_character(character_id)
            .await?;
        let source_inventory = self
            .inventory_repository
            .find_by_character_and_type(character_id, inventory_type.clone())
            .await?;
        let source_slot = self
            .inventory_repository
            .find_slot_by_index(source_inventory.id, slot)
            .await?
            .ok_or_else(|| AppError::NotFound("Inventory slot is empty".to_string()))?;
        let item_instance_id = source_slot.item_instance_id.ok_or_else(|| {
            AppError::BadRequest("Inventory slot has no item instance".to_string())
        })?;
        let item_instance = self
            .item_instance_repository
            .find_by_id(item_instance_id)
            .await?;
        let item = self
            .item_repository
            .find_by_id(item_instance.item_id)
            .await?;
        let catalog_item = shared::data::cities::find_item_by_slug(&item.slug)
            .ok_or_else(|| AppError::Internal(anyhow!("Missing catalog item '{}'", item.slug)))?;
        let target_slot_model = validate_catalog_equip_requirements(catalog_item, &character)?;

        let currently_equipped = self
            .character_repository
            .find_equipped_slot(character_id, target_slot_model.clone())
            .await?;
        let swap_inventory_type = item.inventory_type.to_string();
        let swap_inventory = self
            .inventory_repository
            .find_by_character_and_type(character_id, swap_inventory_type.clone())
            .await?;
        let swap_slot = if currently_equipped.is_some() {
            if swap_inventory.id == source_inventory.id {
                source_slot.slot_index
            } else {
                self.inventory_repository
                    .find_next_available_slot(swap_inventory.id, swap_inventory.capacity)
                    .await?
                    .ok_or_else(|| {
                        AppError::BadRequest(
                            "No inventory space to unequip current item".to_string(),
                        )
                    })?
            }
        } else {
            -1
        };

        self.character_repository
            .equip_inventory_item_transaction(
                character_id,
                source_inventory.id,
                source_slot,
                target_slot_model,
                item_instance_id,
                if currently_equipped.is_some() {
                    Some((swap_inventory.id, swap_slot))
                } else {
                    None
                },
            )
            .await?;

        Ok(())
    }

    pub async fn unequip_item(
        &self,
        character_id: Uuid,
        equipment_slot: String,
    ) -> Result<(), AppError> {
        let equipment_slot_model = equipment_slot
            .parse::<EquipmentSlotModel>()
            .map_err(|_| AppError::BadRequest("Invalid equipment slot".to_string()))?;
        let equipped = self
            .character_repository
            .find_equipped_slot(character_id, equipment_slot_model.clone())
            .await?
            .ok_or_else(|| AppError::NotFound("Equipment slot is empty".to_string()))?;
        let item_instance = self
            .item_instance_repository
            .find_by_id(equipped.item_instance_id)
            .await?;
        let item = self
            .item_repository
            .find_by_id(item_instance.item_id)
            .await?;
        let target_inventory = self
            .inventory_repository
            .find_by_character_and_type(character_id, item.inventory_type.to_string())
            .await?;
        let target_slot = self
            .inventory_repository
            .find_next_available_slot(target_inventory.id, target_inventory.capacity)
            .await?
            .ok_or_else(|| {
                AppError::BadRequest("No inventory space available to unequip item".to_string())
            })?;

        self.character_repository
            .unequip_item_transaction(
                character_id,
                equipment_slot_model,
                target_inventory.id,
                target_slot,
            )
            .await?;

        Ok(())
    }

    pub async fn socket_gem(
        &self,
        character_id: Uuid,
        equipment_slot: String,
        inventory_type: String,
        slot: i16,
        socket_index: i16,
    ) -> Result<(), AppError> {
        let equipment_slot_model = equipment_slot
            .parse::<EquipmentSlotModel>()
            .map_err(|_| AppError::BadRequest("Invalid equipment slot".to_string()))?;
        let equipped = self
            .character_repository
            .find_equipped_slot(character_id, equipment_slot_model)
            .await?
            .ok_or_else(|| AppError::NotFound("Equipment slot is empty".to_string()))?;
        let source_inventory = self
            .inventory_repository
            .find_by_character_and_type(character_id, inventory_type.clone())
            .await?;
        let source_slot = self
            .inventory_repository
            .find_slot_by_index(source_inventory.id, slot)
            .await?
            .ok_or_else(|| AppError::NotFound("Inventory slot is empty".to_string()))?;
        let gem_instance_id = source_slot.item_instance_id.ok_or_else(|| {
            AppError::BadRequest("Inventory slot has no item instance".to_string())
        })?;
        let gem_instance = self
            .item_instance_repository
            .find_by_id(gem_instance_id)
            .await?;
        let gem_item = self
            .item_repository
            .find_by_id(gem_instance.item_id)
            .await?;
        let gem_item_data =
            shared::data::cities::find_item_by_slug(&gem_item.slug).ok_or_else(|| {
                AppError::Internal(anyhow!("Missing catalog item '{}'", gem_item.slug))
            })?;
        if gem_item_data.kind.gem_modifiers().is_none()
            && gem_item_data.kind.gem_effect_rolls().is_none()
        {
            return Err(AppError::BadRequest("Item is not a gem".to_string()));
        }

        let equipment_instance = self
            .item_instance_repository
            .find_by_id(equipped.item_instance_id)
            .await?;
        let equipment_item = self
            .item_repository
            .find_by_id(equipment_instance.item_id)
            .await?;
        let equipment_item_data = shared::data::cities::find_item_by_slug(&equipment_item.slug)
            .ok_or_else(|| {
                AppError::Internal(anyhow!("Missing catalog item '{}'", equipment_item.slug))
            })?;
        let slot_config = equipment_item_data.kind.gem_slots().ok_or_else(|| {
            AppError::BadRequest("Equipped item does not support gems".to_string())
        })?;
        let total_slots = slot_config.base_slots
            + equipment_instance
                .bonus_gem_slots
                .clamp(0, slot_config.max_bonus_slots);
        if socket_index < 0 || socket_index >= total_slots {
            return Err(AppError::BadRequest("Invalid gem socket index".to_string()));
        }

        self.character_repository
            .socket_gem_transaction(
                source_inventory.id,
                source_slot.slot_index,
                equipment_instance.id,
                socket_index,
                gem_instance_id,
            )
            .await?;

        Ok(())
    }
}

pub(crate) fn validate_catalog_equip_requirements(
    catalog_item: &ItemData,
    character: &PlayableCharacterRow,
) -> Result<EquipmentSlotModel, AppError> {
    let target_slot = catalog_item
        .kind
        .equipment_slot()
        .ok_or_else(|| AppError::BadRequest("Item is not equippable".to_string()))?;

    let level_req = match &catalog_item.kind {
        shared::models::item_data::ItemKind::Weapon(data) => data.level_req,
        shared::models::item_data::ItemKind::Armor(data) => data.level_req,
        _ => 0,
    };

    if character.level < level_req {
        return Err(AppError::BadRequest(format!(
            "Character level {} does not meet item requirement {}",
            character.level, level_req
        )));
    }

    if let shared::models::item_data::ItemKind::Weapon(data) = &catalog_item.kind {
        if let Some(restriction) = data.class {
            let matches_base = character.base_character_slug == restriction;
            let matches_current_class = character.current_class_slug == restriction;
            if !matches_base && !matches_current_class {
                return Err(AppError::BadRequest(format!(
                    "Item '{}' is restricted to '{}'",
                    catalog_item.slug, restriction
                )));
            }
        }
    }

    Ok(to_equipment_slot_model(target_slot))
}

pub(crate) fn to_equipment_slot_model(slot: EquipmentSlot) -> EquipmentSlotModel {
    match slot {
        EquipmentSlot::Weapon => EquipmentSlotModel::Weapon,
        EquipmentSlot::Head => EquipmentSlotModel::Head,
        EquipmentSlot::Chest => EquipmentSlotModel::Chest,
        EquipmentSlot::Legs => EquipmentSlotModel::Legs,
        EquipmentSlot::Gloves => EquipmentSlotModel::Gloves,
        EquipmentSlot::Shoes => EquipmentSlotModel::Shoes,
        EquipmentSlot::AccRing1 => EquipmentSlotModel::AccRing1,
        EquipmentSlot::AccRing2 => EquipmentSlotModel::AccRing2,
        EquipmentSlot::AccNecklace => EquipmentSlotModel::AccNecklace,
        EquipmentSlot::AccEarrings => EquipmentSlotModel::AccEarrings,
        EquipmentSlot::AccArm => EquipmentSlotModel::AccArm,
        EquipmentSlot::AccFaceBottom => EquipmentSlotModel::AccFaceBottom,
        EquipmentSlot::AccFaceMiddle => EquipmentSlotModel::AccFaceMiddle,
        EquipmentSlot::AccFaceTop => EquipmentSlotModel::AccFaceTop,
        EquipmentSlot::AccBottomPiece => EquipmentSlotModel::AccBottomPiece,
        EquipmentSlot::AccTopPiece => EquipmentSlotModel::AccTopPiece,
        EquipmentSlot::AccWeapon => EquipmentSlotModel::AccWeapon,
        EquipmentSlot::AccSupportUnit => EquipmentSlotModel::AccSupportUnit,
    }
}
