use crate::{
    db::{Database, schema::items},
    models::item::ItemModel,
    repositories::{Repository, RepositoryError},
};
use diesel::prelude::*;
use shared::models::item::Item;
use uuid::Uuid;

pub struct CreateItemParams {
    pub name: String,
    pub class: Option<String>,
    pub description: Option<String>,
    pub rarity: String,
    pub equipment_slot: Option<String>,
    pub level_req: Option<i16>,
    pub stats: Option<serde_json::Value>,
    pub slug: String,
    pub inventory_type: String,
    pub max_stack: Option<i16>,
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
        let class_model = params.class.and_then(|s| s.parse().ok());
        let equipment_slot_model = params.equipment_slot.and_then(|s| s.parse().ok());
        let max_stack = params.max_stack.unwrap_or(1);

        let model = ItemModel::new(
            params.name.clone(),
            params.description,
            params
                .rarity
                .parse()
                .map_err(|_| RepositoryError::NotFound)?,
            equipment_slot_model,
            class_model,
            params.level_req,
            params.stats,
            params.slug.clone(),
            params
                .inventory_type
                .parse()
                .map_err(|_| RepositoryError::NotFound)?,
            max_stack,
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

    pub async fn find_by_id(&self, item_id: Uuid) -> Result<Item, RepositoryError> {
        self.run_blocking(move |conn| {
            use crate::db::schema::items::dsl::*;
            items
                .find(item_id)
                .first::<ItemModel>(conn)
                .map(|m| m.into())
                .map_err(Into::into)
        })
        .await
    }
}
