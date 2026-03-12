use diesel::prelude::*;

use crate::{
    db::{
        Database,
        schema::{characters, inventory},
    },
    models::{character::CharacterModel, inventory::InventoryModel},
    repositories::{Repository, RepositoryError},
};

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

    pub async fn find_by_id(
        &self,
        character_id: uuid::Uuid,
    ) -> Result<CharacterModel, RepositoryError> {
        self.run_blocking(move |conn| {
            use crate::db::schema::characters::dsl::*;

            characters
                .filter(id.eq(character_id))
                .first::<CharacterModel>(conn)
                .map_err(Into::into)
        })
        .await
    }

    pub async fn find_by_name(
        &self,
        character_name: String,
    ) -> Result<CharacterModel, RepositoryError> {
        self.run_blocking(move |conn| {
            use crate::db::schema::characters::dsl::*;

            characters
                .filter(name.eq(character_name))
                .first::<CharacterModel>(conn)
                .map_err(Into::into)
        })
        .await
    }

    pub async fn create(
        &self,
        account_id: uuid::Uuid,
        params: CreateCharacterParams,
    ) -> Result<CharacterModel, RepositoryError> {
        let model = CharacterModel::new(
            account_id,
            params.name,
            params
                .class
                .parse()
                .map_err(|_| RepositoryError::NotFound)?,
        );

        self.run_blocking(move |conn| {
            use crate::models::inventory_type::InventoryTypeModel::*;

            conn.transaction::<CharacterModel, RepositoryError, _>(|conn| {
                let character: CharacterModel = diesel::insert_into(characters::table)
                    .values(&model)
                    .get_result(conn)?;

                let inventories: Vec<InventoryModel> = [
                    Equipment, Accessory, Consumable, Material, QuestItem, Special,
                ]
                .iter()
                .map(|t| InventoryModel::new(character.id, t.clone(), 56))
                .collect();

                diesel::insert_into(inventory::table)
                    .values(&inventories)
                    .execute(conn)
                    .map(|_| ())?;

                Ok(character)
            })
        })
        .await
    }

    pub async fn count_by_account(&self, acc_id: uuid::Uuid) -> Result<i64, RepositoryError> {
        self.run_blocking(move |conn| {
            use crate::db::schema::characters::dsl::*;
            use diesel::dsl::count_star;

            characters
                .filter(account_id.eq(acc_id))
                .select(count_star())
                .first::<i64>(conn)
                .map_err(Into::into)
        })
        .await
    }

    pub async fn list_all_by_account(
        &self,
        acc_id: uuid::Uuid,
    ) -> Result<Vec<CharacterModel>, RepositoryError> {
        self.run_blocking(move |conn| {
            use crate::db::schema::characters::dsl::*;

            characters
                .filter(account_id.eq(acc_id))
                .load::<CharacterModel>(conn)
                .map_err(Into::into)
        })
        .await
    }
}
