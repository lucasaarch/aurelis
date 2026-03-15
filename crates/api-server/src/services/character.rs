use std::collections::HashMap;

use serde_json::Value;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{
    equipment::EquipmentModel, inventory::InventoryModel, inventory_item::InventoryItemModel,
    item::ItemModel, item_instance::ItemInstanceModel, item_instance_gem::ItemInstanceGemModel,
    player_character::PlayerCharacterModel,
};
use crate::repositories::account::PgAccountRepository;
use crate::repositories::character::{
    CreateCharacterParams, PgCharacterRepository, PlayableCharacterRow,
};
use crate::repositories::item::PgItemRepository;
use shared::models::character_data::CharacterSkillUnlocks;
use shared::models::skill_data::CharacterSkillUnlockTier;

pub struct CreateCharacterInput {
    pub name: String,
    pub class: String,
}

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
    pub inventories: Vec<PersistedInventorySnapshot>,
    pub equipment: Vec<PersistedEquipmentSnapshot>,
    pub item_instances: Vec<PersistedItemInstanceSnapshot>,
}

pub struct PersistedInventorySnapshot {
    pub id: Uuid,
    pub inventory_type: String,
    pub capacity: i16,
    pub items: Vec<PersistedInventoryItemSnapshot>,
}

pub struct PersistedInventoryItemSnapshot {
    pub id: Uuid,
    pub inventory_id: Uuid,
    pub inventory_type: String,
    pub slot_index: i16,
    pub quantity: i16,
    pub item_instance_id: Option<Uuid>,
    pub item_id: Option<Uuid>,
    pub item_slug: Option<String>,
}

pub struct PersistedEquipmentSnapshot {
    pub slot: String,
    pub item_instance_id: Uuid,
}

pub struct PersistedItemInstanceSnapshot {
    pub id: Uuid,
    pub item_id: Uuid,
    pub item_slug: String,
    pub inventory_type: String,
    pub refinement: i16,
    pub bonus_gem_slots: i16,
    pub attributes_json: String,
    pub in_shared_storage: bool,
    pub in_trade: bool,
    pub gems: Vec<PersistedItemInstanceGemSnapshot>,
}

pub struct PersistedItemInstanceGemSnapshot {
    pub slot_index: i16,
    pub gem_instance_id: Uuid,
}

#[derive(Clone)]
pub struct CharacterService {
    character_repository: PgCharacterRepository,
    account_repository: PgAccountRepository,
    item_repository: PgItemRepository,
}

impl CharacterService {
    pub fn new(
        character_repository: PgCharacterRepository,
        account_repository: PgAccountRepository,
        item_repository: PgItemRepository,
    ) -> Self {
        Self {
            character_repository,
            account_repository,
            item_repository,
        }
    }

    pub async fn create(
        &self,
        account_id: Uuid,
        input: CreateCharacterInput,
    ) -> Result<PlayerCharacterModel, AppError> {
        let account = match self.account_repository.find_by_id(account_id).await {
            Ok(a) => a,
            Err(_) => {
                return Err(AppError::Unauthorized(
                    "Unable to fetch account data".to_string(),
                ));
            }
        };

        let max = account.max_characters as i64;
        let current = match self.character_repository.count_by_account(account_id).await {
            Ok(c) => c,
            Err(_) => {
                return Err(AppError::Internal(anyhow::anyhow!(
                    "Unknown Database Error"
                )));
            }
        };

        if current >= max {
            return Err(AppError::BadRequest(
                "Character limit reached for this account".to_string(),
            ));
        }

        self.character_repository
            .create(
                account_id,
                CreateCharacterParams {
                    name: input.name,
                    class: input.class,
                },
            )
            .await
            .map_err(Into::into)
    }

    pub async fn list_all(&self, account_id: Uuid) -> Result<Vec<PlayerCharacterModel>, AppError> {
        match self
            .character_repository
            .list_all_by_account(account_id)
            .await
        {
            Ok(v) => Ok(v),
            Err(_) => Err(AppError::Internal(anyhow::anyhow!("DB error"))),
        }
    }

    pub async fn find_by_name(&self, name: String) -> Result<PlayerCharacterModel, AppError> {
        self.character_repository
            .find_by_name(name)
            .await
            .map_err(Into::into)
    }

    pub async fn verify_ownership(
        &self,
        account_id: Uuid,
        character_id: Uuid,
    ) -> Result<(), AppError> {
        let character = match self.character_repository.find_by_id(character_id).await {
            Ok(c) => c,
            Err(_) => {
                return Err(AppError::Unauthorized(
                    "Unable to fech character data".to_string(),
                ));
            }
        };

        if character.account_id != account_id {
            return Err(AppError::PermissionDenied(
                "Character does not belong to this account".to_string(),
            ));
        }

        Ok(())
    }

    pub async fn load_playable_character(
        &self,
        account_id: Uuid,
        character_id: Uuid,
    ) -> Result<PlayableCharacterSnapshot, AppError> {
        let row = match self
            .character_repository
            .find_playable_character(character_id)
            .await
        {
            Ok(row) => row,
            Err(_) => {
                return Err(AppError::Unauthorized(
                    "Unable to fetch character data".to_string(),
                ));
            }
        };

        if row.account_id != account_id {
            return Err(AppError::PermissionDenied(
                "Character does not belong to this account".to_string(),
            ));
        }

        let inventories = self
            .character_repository
            .list_inventories(character_id)
            .await?;
        let inventory_by_id = inventories
            .iter()
            .map(|inventory| (inventory.id, inventory.clone()))
            .collect::<HashMap<_, _>>();
        let inventory_ids = inventories
            .iter()
            .map(|inventory| inventory.id)
            .collect::<Vec<_>>();
        let inventory_items = self
            .character_repository
            .list_inventory_items(inventory_ids)
            .await?;
        let equipment = self
            .character_repository
            .list_equipment(character_id)
            .await?;

        let mut referenced_item_instance_ids = inventory_items
            .iter()
            .filter_map(|item| item.item_instance_id)
            .collect::<Vec<_>>();
        referenced_item_instance_ids.extend(equipment.iter().map(|item| item.item_instance_id));
        referenced_item_instance_ids.sort_unstable();
        referenced_item_instance_ids.dedup();

        let root_item_instances = self
            .character_repository
            .list_item_instances(referenced_item_instance_ids)
            .await?;
        let root_item_instance_ids = root_item_instances
            .iter()
            .map(|item_instance| item_instance.id)
            .collect::<Vec<_>>();
        let item_instance_gems = self
            .character_repository
            .list_item_instance_gems(root_item_instance_ids)
            .await?;

        let mut gem_instance_ids = item_instance_gems
            .iter()
            .map(|gem| gem.gem_instance_id)
            .collect::<Vec<_>>();
        gem_instance_ids.sort_unstable();
        gem_instance_ids.dedup();

        let gem_item_instances = self
            .character_repository
            .list_item_instances(gem_instance_ids)
            .await?;

        let mut item_instances = root_item_instances;
        item_instances.extend(gem_item_instances);

        let mut item_ids = inventory_items
            .iter()
            .filter_map(|item| item.item_id)
            .collect::<Vec<_>>();
        item_ids.extend(
            item_instances
                .iter()
                .map(|item_instance| item_instance.item_id),
        );
        item_ids.sort_unstable();
        item_ids.dedup();

        let item_by_id = self
            .item_repository
            .list_by_ids(item_ids)
            .await?
            .into_iter()
            .map(|item| (item.id, item))
            .collect::<HashMap<_, _>>();

        Ok(map_playable_character_snapshot(
            row,
            inventories,
            inventory_by_id,
            inventory_items,
            equipment,
            item_instances,
            item_instance_gems,
            item_by_id,
        ))
    }

    pub async fn unlock_skill_tier(
        &self,
        account_id: Uuid,
        character_id: Uuid,
        tier: CharacterSkillUnlockTier,
    ) -> Result<(), AppError> {
        let row = self
            .character_repository
            .find_playable_character(character_id)
            .await
            .map_err(|_| AppError::Unauthorized("Unable to fetch character data".to_string()))?;

        if row.account_id != account_id {
            return Err(AppError::PermissionDenied(
                "Character does not belong to this account".to_string(),
            ));
        }

        let already_unlocked = match tier {
            CharacterSkillUnlockTier::Beginner => row.beginner_skill_unlocked,
            CharacterSkillUnlockTier::Intermediate => row.intermediate_skill_unlocked,
        };
        if already_unlocked {
            return Err(AppError::BadRequest(
                "Skill tier already unlocked for this character".to_string(),
            ));
        }

        let required_level = match tier {
            CharacterSkillUnlockTier::Beginner => 15,
            CharacterSkillUnlockTier::Intermediate => 35,
        };
        if row.level < required_level {
            return Err(AppError::BadRequest(format!(
                "Character level {} is below required level {}",
                row.level, required_level
            )));
        }

        self.character_repository
            .unlock_skill_tier(character_id, tier)
            .await
            .map_err(Into::into)
    }
}

fn map_playable_character_snapshot(
    row: PlayableCharacterRow,
    inventories: Vec<InventoryModel>,
    inventory_by_id: HashMap<Uuid, InventoryModel>,
    inventory_items: Vec<InventoryItemModel>,
    equipment: Vec<EquipmentModel>,
    item_instances: Vec<ItemInstanceModel>,
    item_instance_gems: Vec<ItemInstanceGemModel>,
    item_by_id: HashMap<Uuid, ItemModel>,
) -> PlayableCharacterSnapshot {
    let mut item_instance_gems_by_item_instance_id = item_instance_gems.into_iter().fold(
        HashMap::<Uuid, Vec<ItemInstanceGemModel>>::new(),
        |mut acc, gem| {
            acc.entry(gem.item_instance_id).or_default().push(gem);
            acc
        },
    );

    let item_instances = item_instances
        .into_iter()
        .map(|item_instance| {
            let item = item_by_id
                .get(&item_instance.item_id)
                .expect("missing item catalog for item instance");
            let gems = item_instance_gems_by_item_instance_id
                .remove(&item_instance.id)
                .unwrap_or_default()
                .into_iter()
                .map(|gem| PersistedItemInstanceGemSnapshot {
                    slot_index: gem.slot_index,
                    gem_instance_id: gem.gem_instance_id,
                })
                .collect::<Vec<_>>();

            PersistedItemInstanceSnapshot {
                id: item_instance.id,
                item_id: item_instance.item_id,
                item_slug: item.slug.clone(),
                inventory_type: item.inventory_type.to_string(),
                refinement: item_instance.refinement,
                bonus_gem_slots: item_instance.bonus_gem_slots,
                attributes_json: json_value_to_string(item_instance.attributes),
                in_shared_storage: item_instance.in_shared_storage,
                in_trade: item_instance.in_trade,
                gems,
            }
        })
        .collect::<Vec<_>>();

    let mut inventory_items_by_inventory_id = inventory_items.into_iter().fold(
        HashMap::<Uuid, Vec<PersistedInventoryItemSnapshot>>::new(),
        |mut acc, item| {
            let item_slug = item
                .item_id
                .and_then(|item_id| item_by_id.get(&item_id).map(|item| item.slug.clone()))
                .or_else(|| {
                    item.item_instance_id.and_then(|item_instance_id| {
                        item_instances
                            .iter()
                            .find(|instance| instance.id == item_instance_id)
                            .map(|instance| instance.item_slug.clone())
                    })
                });

            let inventory = inventory_by_id
                .get(&item.inventory_id)
                .expect("missing inventory for inventory item");

            acc.entry(item.inventory_id)
                .or_default()
                .push(PersistedInventoryItemSnapshot {
                    id: item.id,
                    inventory_id: item.inventory_id,
                    inventory_type: inventory.inventory_type.to_string(),
                    slot_index: item.slot_index,
                    quantity: item.quantity,
                    item_instance_id: item.item_instance_id,
                    item_id: item.item_id,
                    item_slug,
                });
            acc
        },
    );

    let inventories = inventories
        .into_iter()
        .map(|inventory| PersistedInventorySnapshot {
            id: inventory.id,
            inventory_type: inventory.inventory_type.to_string(),
            capacity: inventory.capacity,
            items: inventory_items_by_inventory_id
                .remove(&inventory.id)
                .unwrap_or_default(),
        })
        .collect::<Vec<_>>();

    let equipment = equipment
        .into_iter()
        .map(|equipment| PersistedEquipmentSnapshot {
            slot: equipment.slot.to_string(),
            item_instance_id: equipment.item_instance_id,
        })
        .collect::<Vec<_>>();

    PlayableCharacterSnapshot {
        account_id: row.account_id,
        character_id: row.character_id,
        name: row.name,
        base_character_slug: row.base_character_slug,
        current_class_slug: row.current_class_slug,
        level: row.level,
        experience: row.experience,
        credits: row.credits,
        skill_unlocks: CharacterSkillUnlocks {
            beginner: row.beginner_skill_unlocked,
            intermediate: row.intermediate_skill_unlocked,
        },
        inventories,
        equipment,
        item_instances,
    }
}

fn json_value_to_string(value: Value) -> String {
    match value {
        Value::String(value) => value,
        other => other.to_string(),
    }
}
