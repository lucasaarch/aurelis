use shared::models::{character::Character};
use diesel::prelude::*;

use crate::{db::{Database, schema::characters}, models::character::CharacterModel, repositories::{Repository, RepositoryError}};


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

    pub async fn create(&self, account_id: uuid::Uuid, params: CreateCharacterParams) -> Result<Character, RepositoryError> {
        let model = CharacterModel::new(
            account_id,
            params.name,
            params.class.parse().map_err(|_| RepositoryError::NotFound)?,
        );

        self.run_blocking(move |conn| {
            diesel::insert_into(characters::table)
                .values(&model)
                .get_result(conn)
                .map(|c: CharacterModel| c.into())
                .map_err(Into::into)
        })
        .await
    }
}