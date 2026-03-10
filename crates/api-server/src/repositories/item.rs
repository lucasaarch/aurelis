use crate::{
    db::{Database, schema::items},
    models::item::ItemModel,
    repositories::{Repository, RepositoryError},
};
use diesel::prelude::*;
use shared::models::item::Item;

pub struct CreateItemParams {
    pub slug: String,
    pub name: String,
    pub class: Option<String>,
    pub description: Option<String>,
    pub rarity: String,
    pub equipment_slot: Option<String>,
    pub level_req: i16,
    pub stats: serde_json::Value,
}

#[derive(Clone)]
pub struct PgItemRepository {
    db: Database,
}

impl Repository for PgItemRepository {
    fn db(&self) -> Database {
        self.db.clone()
    }
}

impl PgItemRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn create(&self, params: CreateItemParams) -> Result<Item, RepositoryError> {
        // Convert strings to enum models where needed inside ItemModel::new
        let class_model = params.class.and_then(|s| s.parse().ok());
        let equipment_slot_model = params.equipment_slot.and_then(|s| s.parse().ok());

        // Map rarity string to ItemRarityModel via parsing in ItemModel::new call
        let model = ItemModel::new(
            params.name.clone(),
            params.description,
            params.rarity.parse().map_err(|_| RepositoryError::NotFound)?,
            equipment_slot_model,
            class_model,
            params.level_req,
            params.stats,
            params.slug.clone(),
        );

        self.run_blocking(move |conn| {
            diesel::insert_into(items::table)
                .values(&model)
                .get_result(conn)
                .map(|m: ItemModel| m.into())
                .map_err(Into::into)
        })
        .await
    }
}
