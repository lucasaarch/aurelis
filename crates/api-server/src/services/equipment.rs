use anyhow::anyhow;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::equipment_slot::EquipmentSlotModel,
    repositories::{
        character::PgCharacterRepository, inventory::PgInventoryRepository, item::PgItemRepository,
        item_instance::PgItemInstanceRepository,
    },
};

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
        let target_slot = catalog_item
            .kind
            .equipment_slot()
            .ok_or_else(|| AppError::BadRequest("Item is not equippable".to_string()))?;
        let target_slot_model = match target_slot {
            shared::models::equipment_slot::EquipmentSlot::Weapon => EquipmentSlotModel::Weapon,
            shared::models::equipment_slot::EquipmentSlot::Head => EquipmentSlotModel::Head,
            shared::models::equipment_slot::EquipmentSlot::Chest => EquipmentSlotModel::Chest,
            shared::models::equipment_slot::EquipmentSlot::Legs => EquipmentSlotModel::Legs,
            shared::models::equipment_slot::EquipmentSlot::Gloves => EquipmentSlotModel::Gloves,
            shared::models::equipment_slot::EquipmentSlot::Shoes => EquipmentSlotModel::Shoes,
            shared::models::equipment_slot::EquipmentSlot::AccRing1 => EquipmentSlotModel::AccRing1,
            shared::models::equipment_slot::EquipmentSlot::AccRing2 => EquipmentSlotModel::AccRing2,
            shared::models::equipment_slot::EquipmentSlot::AccNecklace => {
                EquipmentSlotModel::AccNecklace
            }
            shared::models::equipment_slot::EquipmentSlot::AccEarrings => {
                EquipmentSlotModel::AccEarrings
            }
            shared::models::equipment_slot::EquipmentSlot::AccArm => EquipmentSlotModel::AccArm,
            shared::models::equipment_slot::EquipmentSlot::AccFaceBottom => {
                EquipmentSlotModel::AccFaceBottom
            }
            shared::models::equipment_slot::EquipmentSlot::AccFaceMiddle => {
                EquipmentSlotModel::AccFaceMiddle
            }
            shared::models::equipment_slot::EquipmentSlot::AccFaceTop => {
                EquipmentSlotModel::AccFaceTop
            }
            shared::models::equipment_slot::EquipmentSlot::AccBottomPiece => {
                EquipmentSlotModel::AccBottomPiece
            }
            shared::models::equipment_slot::EquipmentSlot::AccTopPiece => {
                EquipmentSlotModel::AccTopPiece
            }
            shared::models::equipment_slot::EquipmentSlot::AccWeapon => {
                EquipmentSlotModel::AccWeapon
            }
            shared::models::equipment_slot::EquipmentSlot::AccSupportUnit => {
                EquipmentSlotModel::AccSupportUnit
            }
        };

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
            self.inventory_repository
                .find_next_available_slot(swap_inventory.id, swap_inventory.capacity)
                .await?
                .ok_or_else(|| {
                    AppError::BadRequest("No inventory space to unequip current item".to_string())
                })?
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
}
