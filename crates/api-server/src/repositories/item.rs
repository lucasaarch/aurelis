use crate::{
    db::Database,
    models::item::ItemModel,
    repositories::{Repository, RepositoryError},
};
use diesel::pg::Pg;
use diesel::prelude::*;
use uuid::Uuid;

use crate::models::{
    character_class::CharacterClassModel, equipment_slot::EquipmentSlotModel,
    inventory_type::InventoryTypeModel, item_rarity::ItemRarityModel,
};

#[derive(Clone, Default)]
pub struct ListItemFilters {
    pub class: Option<CharacterClassModel>,
    pub rarity: Option<ItemRarityModel>,
    pub equipment_slot: Option<EquipmentSlotModel>,
    pub inventory_type: Option<InventoryTypeModel>,
    pub level_min: Option<i16>,
    pub level_max: Option<i16>,
    pub search: Option<String>,
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

    pub async fn find_by_id(&self, item_id: Uuid) -> Result<ItemModel, RepositoryError> {
        self.run_blocking(move |conn| {
            use crate::db::schema::items::dsl::*;
            items
                .find(item_id)
                .first::<ItemModel>(conn)
                .map_err(Into::into)
        })
        .await
    }

    pub async fn find_by_slug(&self, item_slug: String) -> Result<ItemModel, RepositoryError> {
        self.run_blocking(move |conn| {
            use crate::db::schema::items::dsl::*;
            items
                .filter(slug.eq(item_slug))
                .first::<ItemModel>(conn)
                .map_err(Into::into)
        })
        .await
    }

    // NOTE: item creation/deletion removed.

    pub async fn list(
        &self,
        page: i64,
        limit: i64,
        filters: ListItemFilters,
    ) -> Result<(Vec<ItemModel>, i64), RepositoryError> {
        let offset = (page - 1) * limit;
        self.run_blocking(move |conn| {
            use crate::db::schema::items::dsl::*;

            let mut count_query = items.into_boxed::<Pg>();
            if let Some(ref value) = filters.class {
                count_query = count_query.filter(class.eq(Some(value.clone())));
            }
            if let Some(ref value) = filters.rarity {
                count_query = count_query.filter(rarity.eq(value.clone()));
            }
            if let Some(ref value) = filters.equipment_slot {
                count_query = count_query.filter(equipment_slot.eq(Some(value.clone())));
            }
            if let Some(ref value) = filters.inventory_type {
                count_query = count_query.filter(inventory_type.eq(value.clone()));
            }
            if let Some(value) = filters.level_min {
                count_query = count_query.filter(level_req.ge(value));
            }
            if let Some(value) = filters.level_max {
                count_query = count_query.filter(level_req.le(value));
            }
            if let Some(ref value) = filters.search {
                let pattern = format!("%{}%", value);
                count_query = count_query.filter(name.ilike(pattern));
            }

            let total = count_query.count().get_result::<i64>(conn)?;

            let mut rows_query = items.into_boxed::<Pg>();
            if let Some(ref value) = filters.class {
                rows_query = rows_query.filter(class.eq(Some(value.clone())));
            }
            if let Some(ref value) = filters.rarity {
                rows_query = rows_query.filter(rarity.eq(value.clone()));
            }
            if let Some(ref value) = filters.equipment_slot {
                rows_query = rows_query.filter(equipment_slot.eq(Some(value.clone())));
            }
            if let Some(ref value) = filters.inventory_type {
                rows_query = rows_query.filter(inventory_type.eq(value.clone()));
            }
            if let Some(value) = filters.level_min {
                rows_query = rows_query.filter(level_req.ge(value));
            }
            if let Some(value) = filters.level_max {
                rows_query = rows_query.filter(level_req.le(value));
            }
            if let Some(ref value) = filters.search {
                let pattern = format!("%{}%", value);
                rows_query = rows_query.filter(name.ilike(pattern));
            }

            let rows = rows_query
                .order(created_at.desc())
                .limit(limit)
                .offset(offset)
                .load::<ItemModel>(conn)?;

            Ok((rows, total))
        })
        .await
    }

    pub async fn list_by_ids(&self, ids: Vec<Uuid>) -> Result<Vec<ItemModel>, RepositoryError> {
        // Fast path for empty input
        if ids.is_empty() {
            return Ok(vec![]);
        }

        self.run_blocking(move |conn| {
            use crate::db::schema::items::dsl::*;
            items
                .filter(id.eq_any(ids))
                .load::<ItemModel>(conn)
                .map_err(Into::into)
        })
        .await
    }
}
