use diesel::dsl::{count_star, insert_into};
use diesel::prelude::*;
use diesel::upsert::excluded;
use uuid::Uuid;

use crate::{
    db::{
        Database,
        schema::{
            character_class_path_classes, character_class_paths, characters, inventory,
            player_characters,
        },
    },
    models::{
        character::CharacterModel, character_class_path::CharacterClassPathModel,
        character_class_path_class::CharacterClassPathClassModel, inventory::InventoryModel,
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
                ))
                .first::<(Uuid, Uuid, String, String, String, i16)>(conn)?;

            Ok(PlayableCharacterRow {
                character_id: row.0,
                account_id: row.1,
                name: row.2,
                base_character_slug: row.3,
                current_class_slug: row.4,
                level: row.5,
            })
        })
        .await
    }
}
