use diesel::dsl::{count_star, insert_into};
use diesel::prelude::*;
use diesel::upsert::excluded;
use shared::models::skill_data::CharacterSkillUnlockTier;
use uuid::Uuid;

use crate::{
    db::{
        Database,
        schema::{
            character_class_path_classes, character_class_paths, characters, equipment, inventory,
            inventory_items, item_instance_gems, item_instances, player_characters,
        },
    },
    models::{
        character::CharacterModel, character_class_path::CharacterClassPathModel,
        character_class_path_class::CharacterClassPathClassModel, equipment::EquipmentModel,
        inventory::InventoryModel, inventory_item::InventoryItemModel,
        item_instance::ItemInstanceModel, item_instance_gem::ItemInstanceGemModel,
        player_character::PlayerCharacterModel,
    },
    repositories::{Repository, RepositoryError},
};
use shared::models::character_data::CharacterData;

#[derive(Clone, Copy)]
pub struct SyncedCharacterClassPath {
    pub id: Uuid,
    pub character_slug: &'static str,
    pub path_index: usize,
}

pub struct PlayerCharacterIntegrityRow {
    pub base_character_slug: String,
    pub current_class_slug: String,
}

pub struct PlayableCharacterRow {
    pub character_id: Uuid,
    pub account_id: Uuid,
    pub name: String,
    pub base_character_slug: String,
    pub current_class_slug: String,
    pub level: i16,
    pub experience: i64,
    pub credits: i64,
    pub beginner_skill_unlocked: bool,
    pub intermediate_skill_unlocked: bool,
}

pub struct CreateCharacterParams {
    pub name: String,
    pub class: String,
}

#[derive(Clone)]
pub struct PgCharacterRepository {
    db: Database,
}

impl Repository for PgCharacterRepository {
    fn db(&self) -> Database {
        self.db.clone()
    }
}

impl PgCharacterRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn find_catalog_by_slug(
        &self,
        character_slug: String,
    ) -> Result<CharacterModel, RepositoryError> {
        self.run_blocking(move |conn| {
            use crate::db::schema::characters::dsl::*;

            characters
                .filter(slug.eq(character_slug))
                .first::<CharacterModel>(conn)
                .map_err(Into::into)
        })
        .await
    }

    pub async fn find_by_id(
        &self,
        player_character_id: Uuid,
    ) -> Result<PlayerCharacterModel, RepositoryError> {
        self.run_blocking(move |conn| {
            use crate::db::schema::player_characters::dsl::*;

            player_characters
                .filter(id.eq(player_character_id))
                .select(PlayerCharacterModel::as_select())
                .first::<PlayerCharacterModel>(conn)
                .map_err(Into::into)
        })
        .await
    }

    pub async fn find_by_name(
        &self,
        player_character_name: String,
    ) -> Result<PlayerCharacterModel, RepositoryError> {
        self.run_blocking(move |conn| {
            use crate::db::schema::player_characters::dsl::*;

            player_characters
                .filter(name.eq(player_character_name))
                .select(PlayerCharacterModel::as_select())
                .first::<PlayerCharacterModel>(conn)
                .map_err(Into::into)
        })
        .await
    }

    pub async fn create(
        &self,
        account: Uuid,
        params: CreateCharacterParams,
    ) -> Result<PlayerCharacterModel, RepositoryError> {
        self.run_blocking(move |conn| {
            use crate::models::inventory_type::InventoryTypeModel::*;

            conn.transaction::<PlayerCharacterModel, RepositoryError, _>(|conn| {
                let catalog_character = characters::table
                    .filter(characters::slug.eq(&params.class))
                    .first::<CharacterModel>(conn)?;

                let model = PlayerCharacterModel::new(
                    account,
                    params.name,
                    catalog_character.id,
                    catalog_character.slug.clone(),
                );

                let created = insert_into(player_characters::table)
                    .values(&model)
                    .returning(PlayerCharacterModel::as_returning())
                    .get_result::<PlayerCharacterModel>(conn)?;

                let inventories: Vec<InventoryModel> = [
                    Equipment, Accessory, Consumable, Material, QuestItem, Special,
                ]
                .iter()
                .map(|t| InventoryModel::new(created.id, t.clone(), 56))
                .collect();

                insert_into(inventory::table)
                    .values(&inventories)
                    .execute(conn)
                    .map(|_| ())?;

                Ok(created)
            })
        })
        .await
    }

    pub async fn count_by_account(&self, acc_id: Uuid) -> Result<i64, RepositoryError> {
        self.run_blocking(move |conn| {
            use crate::db::schema::player_characters::dsl::*;

            player_characters
                .filter(account_id.eq(acc_id))
                .select(count_star())
                .first::<i64>(conn)
                .map_err(Into::into)
        })
        .await
    }

    pub async fn list_all_by_account(
        &self,
        acc_id: Uuid,
    ) -> Result<Vec<PlayerCharacterModel>, RepositoryError> {
        self.run_blocking(move |conn| {
            use crate::db::schema::player_characters::dsl::*;

            player_characters
                .filter(account_id.eq(acc_id))
                .order(created_at.asc())
                .select(PlayerCharacterModel::as_select())
                .load::<PlayerCharacterModel>(conn)
                .map_err(Into::into)
        })
        .await
    }

    pub async fn upsert_catalog_character(
        &self,
        character_slug: &'static str,
    ) -> Result<CharacterModel, RepositoryError> {
        self.run_blocking(move |conn| {
            let model = CharacterModel::new(character_slug.to_string());

            insert_into(characters::table)
                .values(&model)
                .on_conflict(characters::slug)
                .do_update()
                .set(characters::slug.eq(excluded(characters::slug)))
                .get_result::<CharacterModel>(conn)
                .map_err(Into::into)
        })
        .await
    }

    pub async fn list_catalog_slugs(&self) -> Result<Vec<String>, RepositoryError> {
        self.run_blocking(move |conn| {
            use crate::db::schema::characters::dsl::*;

            characters
                .select(slug)
                .load::<String>(conn)
                .map_err(Into::into)
        })
        .await
    }

    pub async fn replace_catalog_character_class_paths(
        &self,
        all_characters: &[&'static CharacterData],
    ) -> Result<Vec<SyncedCharacterClassPath>, RepositoryError> {
        let all_characters = all_characters.to_vec();
        self.run_blocking(move |conn| {
            conn.transaction::<Vec<SyncedCharacterClassPath>, RepositoryError, _>(|conn| {
                diesel::delete(character_class_path_classes::table).execute(conn)?;
                diesel::delete(character_class_paths::table).execute(conn)?;

                let mut synced_paths = Vec::new();

                for character in all_characters {
                    let catalog_character = characters::table
                        .filter(characters::slug.eq(character.slug))
                        .first::<CharacterModel>(conn)?;

                    for (path_index, _) in character.evolution_lines.iter().enumerate() {
                        let model = CharacterClassPathModel::new(catalog_character.id);
                        let created = insert_into(character_class_paths::table)
                            .values(&model)
                            .get_result::<CharacterClassPathModel>(conn)?;

                        synced_paths.push(SyncedCharacterClassPath {
                            id: created.id,
                            character_slug: character.slug,
                            path_index,
                        });
                    }
                }

                Ok(synced_paths)
            })
        })
        .await
    }

    pub async fn replace_catalog_character_class_path_classes(
        &self,
        all_characters: &[&'static CharacterData],
        synced_paths: Vec<SyncedCharacterClassPath>,
    ) -> Result<(), RepositoryError> {
        let all_characters = all_characters.to_vec();
        self.run_blocking(move |conn| {
            conn.transaction::<(), RepositoryError, _>(|conn| {
                for character in all_characters {
                    for (path_index, path) in character.evolution_lines.iter().enumerate() {
                        let synced_path = synced_paths
                            .iter()
                            .find(|value| {
                                value.character_slug == character.slug
                                    && value.path_index == path_index
                            })
                            .ok_or_else(|| {
                                RepositoryError::Internal(format!(
                                    "Missing synced class path for character '{}' at index {}",
                                    character.slug, path_index
                                ))
                            })?;

                        for class in path.steps {
                            let model = CharacterClassPathClassModel::new(
                                class.slug.to_string(),
                                synced_path.id,
                            );

                            insert_into(character_class_path_classes::table)
                                .values(&model)
                                .on_conflict(character_class_path_classes::slug)
                                .do_update()
                                .set(character_class_path_classes::character_class_path_id.eq(
                                    excluded(character_class_path_classes::character_class_path_id),
                                ))
                                .execute(conn)?;
                        }
                    }
                }

                Ok(())
            })
        })
        .await
    }

    pub async fn list_player_character_integrity_rows(
        &self,
    ) -> Result<Vec<PlayerCharacterIntegrityRow>, RepositoryError> {
        self.run_blocking(move |conn| {
            let rows = player_characters::table
                .inner_join(
                    characters::table.on(characters::id.eq(player_characters::character_id)),
                )
                .select((characters::slug, player_characters::current_class_slug))
                .load::<(String, String)>(conn)?;

            Ok(rows
                .into_iter()
                .map(
                    |(base_character_slug, current_class_slug)| PlayerCharacterIntegrityRow {
                        base_character_slug,
                        current_class_slug,
                    },
                )
                .collect())
        })
        .await
    }

    pub async fn find_playable_character(
        &self,
        player_character_id: Uuid,
    ) -> Result<PlayableCharacterRow, RepositoryError> {
        self.run_blocking(move |conn| {
            let row = player_characters::table
                .inner_join(
                    characters::table.on(characters::id.eq(player_characters::character_id)),
                )
                .filter(player_characters::id.eq(player_character_id))
                .select((
                    player_characters::id,
                    player_characters::account_id,
                    player_characters::name,
                    characters::slug,
                    player_characters::current_class_slug,
                    player_characters::level,
                    player_characters::experience,
                    player_characters::credits,
                    player_characters::beginner_skill_unlocked,
                    player_characters::intermediate_skill_unlocked,
                ))
                .first::<(
                    Uuid,
                    Uuid,
                    String,
                    String,
                    String,
                    i16,
                    i64,
                    i64,
                    bool,
                    bool,
                )>(conn)?;

            Ok(PlayableCharacterRow {
                character_id: row.0,
                account_id: row.1,
                name: row.2,
                base_character_slug: row.3,
                current_class_slug: row.4,
                level: row.5,
                experience: row.6,
                credits: row.7,
                beginner_skill_unlocked: row.8,
                intermediate_skill_unlocked: row.9,
            })
        })
        .await
    }

    pub async fn list_inventories(
        &self,
        player_character_id: Uuid,
    ) -> Result<Vec<InventoryModel>, RepositoryError> {
        self.run_blocking(move |conn| {
            inventory::table
                .filter(inventory::character_id.eq(player_character_id))
                .order(inventory::created_at.asc())
                .load::<InventoryModel>(conn)
                .map_err(Into::into)
        })
        .await
    }

    pub async fn list_inventory_items(
        &self,
        inventory_ids: Vec<Uuid>,
    ) -> Result<Vec<InventoryItemModel>, RepositoryError> {
        if inventory_ids.is_empty() {
            return Ok(vec![]);
        }

        self.run_blocking(move |conn| {
            inventory_items::table
                .filter(inventory_items::inventory_id.eq_any(inventory_ids))
                .order((
                    inventory_items::inventory_id.asc(),
                    inventory_items::slot_index.asc(),
                ))
                .load::<InventoryItemModel>(conn)
                .map_err(Into::into)
        })
        .await
    }

    pub async fn list_equipment(
        &self,
        player_character_id: Uuid,
    ) -> Result<Vec<EquipmentModel>, RepositoryError> {
        self.run_blocking(move |conn| {
            equipment::table
                .filter(equipment::character_id.eq(player_character_id))
                .load::<EquipmentModel>(conn)
                .map_err(Into::into)
        })
        .await
    }

    pub async fn list_item_instances(
        &self,
        item_instance_ids: Vec<Uuid>,
    ) -> Result<Vec<ItemInstanceModel>, RepositoryError> {
        if item_instance_ids.is_empty() {
            return Ok(vec![]);
        }

        self.run_blocking(move |conn| {
            item_instances::table
                .filter(item_instances::id.eq_any(item_instance_ids))
                .load::<ItemInstanceModel>(conn)
                .map_err(Into::into)
        })
        .await
    }

    pub async fn list_item_instance_gems(
        &self,
        item_instance_ids: Vec<Uuid>,
    ) -> Result<Vec<ItemInstanceGemModel>, RepositoryError> {
        if item_instance_ids.is_empty() {
            return Ok(vec![]);
        }

        self.run_blocking(move |conn| {
            item_instance_gems::table
                .filter(item_instance_gems::item_instance_id.eq_any(item_instance_ids))
                .order((
                    item_instance_gems::item_instance_id.asc(),
                    item_instance_gems::slot_index.asc(),
                ))
                .load::<ItemInstanceGemModel>(conn)
                .map_err(Into::into)
        })
        .await
    }

    pub async fn unlock_skill_tier(
        &self,
        player_character_id: Uuid,
        tier: CharacterSkillUnlockTier,
    ) -> Result<(), RepositoryError> {
        self.run_blocking(move |conn| {
            let changes = match tier {
                CharacterSkillUnlockTier::Beginner => diesel::update(
                    player_characters::table.filter(player_characters::id.eq(player_character_id)),
                )
                .set(player_characters::beginner_skill_unlocked.eq(true))
                .execute(conn)?,
                CharacterSkillUnlockTier::Intermediate => diesel::update(
                    player_characters::table.filter(player_characters::id.eq(player_character_id)),
                )
                .set(player_characters::intermediate_skill_unlocked.eq(true))
                .execute(conn)?,
            };

            if changes == 0 {
                return Err(RepositoryError::NotFound);
            }

            Ok(())
        })
        .await
    }

    pub async fn find_equipped_slot(
        &self,
        player_character_id: Uuid,
        slot_value: crate::models::equipment_slot::EquipmentSlotModel,
    ) -> Result<Option<EquipmentModel>, RepositoryError> {
        self.run_blocking(move |conn| {
            equipment::table
                .filter(equipment::character_id.eq(player_character_id))
                .filter(equipment::slot.eq(slot_value))
                .first::<EquipmentModel>(conn)
                .optional()
                .map_err(Into::into)
        })
        .await
    }

    pub async fn equip_item_instance(
        &self,
        player_character_id: Uuid,
        slot_value: crate::models::equipment_slot::EquipmentSlotModel,
        item_instance_id_value: Uuid,
    ) -> Result<(), RepositoryError> {
        self.run_blocking(move |conn| {
            conn.transaction::<(), RepositoryError, _>(|conn| {
                diesel::insert_into(equipment::table)
                    .values(&EquipmentModel::new(
                        player_character_id,
                        slot_value.clone(),
                        item_instance_id_value,
                    ))
                    .on_conflict((equipment::character_id, equipment::slot))
                    .do_update()
                    .set((
                        equipment::item_instance_id.eq(item_instance_id_value),
                        equipment::equipped_at.eq(diesel::dsl::now),
                    ))
                    .execute(conn)?;
                Ok(())
            })
        })
        .await
    }

    pub async fn unequip_slot(
        &self,
        player_character_id: Uuid,
        slot_value: crate::models::equipment_slot::EquipmentSlotModel,
    ) -> Result<(), RepositoryError> {
        self.run_blocking(move |conn| {
            diesel::delete(equipment::table)
                .filter(equipment::character_id.eq(player_character_id))
                .filter(equipment::slot.eq(slot_value))
                .execute(conn)
                .map(|_| ())
                .map_err(Into::into)
        })
        .await
    }

    pub async fn equip_inventory_item_transaction(
        &self,
        player_character_id: Uuid,
        source_inventory_id: Uuid,
        source_slot: InventoryItemModel,
        target_slot_value: crate::models::equipment_slot::EquipmentSlotModel,
        item_instance_id_value: Uuid,
        swap_target: Option<(Uuid, i16)>,
    ) -> Result<(), RepositoryError> {
        self.run_blocking(move |conn| {
            conn.transaction::<(), RepositoryError, _>(|conn| {
                let currently_equipped = equipment::table
                    .filter(equipment::character_id.eq(player_character_id))
                    .filter(equipment::slot.eq(target_slot_value.clone()))
                    .first::<EquipmentModel>(conn)
                    .optional()?;

                diesel::delete(inventory_items::table)
                    .filter(inventory_items::inventory_id.eq(source_inventory_id))
                    .filter(inventory_items::slot_index.eq(source_slot.slot_index))
                    .execute(conn)?;

                if let Some(equipped) = currently_equipped {
                    let (dest_inventory_id, dest_slot) = swap_target.ok_or_else(|| {
                        RepositoryError::Internal(
                            "Missing swap target for occupied equipment slot".to_string(),
                        )
                    })?;

                    let swapped_slot = InventoryItemModel::new(
                        dest_inventory_id,
                        Some(equipped.item_instance_id),
                        None,
                        dest_slot,
                        1,
                    );

                    diesel::insert_into(inventory_items::table)
                        .values(&swapped_slot)
                        .execute(conn)?;
                }

                diesel::insert_into(equipment::table)
                    .values(&EquipmentModel::new(
                        player_character_id,
                        target_slot_value.clone(),
                        item_instance_id_value,
                    ))
                    .on_conflict((equipment::character_id, equipment::slot))
                    .do_update()
                    .set((
                        equipment::item_instance_id.eq(item_instance_id_value),
                        equipment::equipped_at.eq(diesel::dsl::now),
                    ))
                    .execute(conn)?;

                Ok(())
            })
        })
        .await
    }

    pub async fn unequip_item_transaction(
        &self,
        player_character_id: Uuid,
        equipment_slot_value: crate::models::equipment_slot::EquipmentSlotModel,
        target_inventory_id: Uuid,
        target_slot: i16,
    ) -> Result<(), RepositoryError> {
        self.run_blocking(move |conn| {
            conn.transaction::<(), RepositoryError, _>(|conn| {
                let equipped = equipment::table
                    .filter(equipment::character_id.eq(player_character_id))
                    .filter(equipment::slot.eq(equipment_slot_value.clone()))
                    .first::<EquipmentModel>(conn)?;

                diesel::delete(equipment::table)
                    .filter(equipment::character_id.eq(player_character_id))
                    .filter(equipment::slot.eq(equipment_slot_value))
                    .execute(conn)?;

                let inventory_slot = InventoryItemModel::new(
                    target_inventory_id,
                    Some(equipped.item_instance_id),
                    None,
                    target_slot,
                    1,
                );

                diesel::insert_into(inventory_items::table)
                    .values(&inventory_slot)
                    .execute(conn)?;

                Ok(())
            })
        })
        .await
    }

    pub async fn socket_gem_transaction(
        &self,
        source_inventory_id: Uuid,
        source_slot_index: i16,
        item_instance_id: Uuid,
        socket_index: i16,
        gem_instance_id: Uuid,
    ) -> Result<(), RepositoryError> {
        self.run_blocking(move |conn| {
            conn.transaction::<(), RepositoryError, _>(|conn| {
                let existing = item_instance_gems::table
                    .filter(item_instance_gems::item_instance_id.eq(item_instance_id))
                    .filter(item_instance_gems::slot_index.eq(socket_index))
                    .first::<ItemInstanceGemModel>(conn)
                    .optional()?;

                diesel::delete(inventory_items::table)
                    .filter(inventory_items::inventory_id.eq(source_inventory_id))
                    .filter(inventory_items::slot_index.eq(source_slot_index))
                    .execute(conn)?;

                if let Some(existing) = existing {
                    diesel::delete(
                        item_instance_gems::table.filter(item_instance_gems::id.eq(existing.id)),
                    )
                    .execute(conn)?;

                    diesel::delete(
                        item_instances::table
                            .filter(item_instances::id.eq(existing.gem_instance_id)),
                    )
                    .execute(conn)?;
                }

                diesel::insert_into(item_instance_gems::table)
                    .values(&ItemInstanceGemModel::new(
                        item_instance_id,
                        socket_index,
                        gem_instance_id,
                    ))
                    .execute(conn)?;

                Ok(())
            })
        })
        .await
    }
}
