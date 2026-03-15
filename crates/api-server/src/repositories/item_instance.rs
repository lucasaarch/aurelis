use diesel::prelude::*;
use serde_json::Value;
use uuid::Uuid;

use crate::{
    db::{Database, schema::item_instances},
    models::item_instance::ItemInstanceModel,
    repositories::{Repository, RepositoryError},
};

#[derive(Clone)]
pub struct PgItemInstanceRepository {
    db: Database,
}

impl Repository for PgItemInstanceRepository {
    fn db(&self) -> Database {
        self.db.clone()
    }
}

impl PgItemInstanceRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn find_by_id(
        &self,
        item_instance_id: Uuid,
    ) -> Result<ItemInstanceModel, RepositoryError> {
        self.run_blocking(move |conn| {
            item_instances::table
                .find(item_instance_id)
                .first::<ItemInstanceModel>(conn)
                .map_err(Into::into)
        })
        .await
    }

    pub async fn update_state(
        &self,
        item_instance_id: Uuid,
        bonus_gem_slots: i16,
        attributes: Value,
    ) -> Result<(), RepositoryError> {
        self.run_blocking(move |conn| {
            diesel::update(item_instances::table.find(item_instance_id))
                .set((
                    item_instances::bonus_gem_slots.eq(bonus_gem_slots),
                    item_instances::attributes.eq(attributes),
                    item_instances::updated_at.eq(diesel::dsl::now),
                ))
                .execute(conn)
                .map(|_| ())
                .map_err(Into::into)
        })
        .await
    }
}
