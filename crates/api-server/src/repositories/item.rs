use crate::{
    db::Database,
    models::item::ItemModel,
    repositories::{Repository, RepositoryError},
};
use diesel::dsl::insert_into;
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::upsert::excluded;
use uuid::Uuid;

use crate::models::inventory_type::InventoryTypeModel;

#[derive(Clone, Default)]
pub struct ListItemFilters {
    pub inventory_type: Option<InventoryTypeModel>,
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

    pub async fn upsert_catalog_item(
        &self,
        item_slug: String,
        item_inventory_type: InventoryTypeModel,
    ) -> Result<ItemModel, RepositoryError> {
        self.run_blocking(move |conn| {
            use crate::db::schema::items::dsl::*;

            let item = ItemModel::new(item_slug, item_inventory_type);

            insert_into(items)
                .values(&item)
                .on_conflict(slug)
                .do_update()
                .set(inventory_type.eq(excluded(inventory_type)))
                .get_result::<ItemModel>(conn)
                .map_err(Into::into)
        })
        .await
    }

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
            if let Some(ref value) = filters.inventory_type {
                count_query = count_query.filter(inventory_type.eq(value.clone()));
            }
            if let Some(ref value) = filters.search {
                let pattern = format!("%{}%", value);
                count_query = count_query.filter(slug.ilike(pattern));
            }

            let total = count_query.count().get_result::<i64>(conn)?;

            let mut rows_query = items.into_boxed::<Pg>();
            if let Some(ref value) = filters.inventory_type {
                rows_query = rows_query.filter(inventory_type.eq(value.clone()));
            }
            if let Some(ref value) = filters.search {
                let pattern = format!("%{}%", value);
                rows_query = rows_query.filter(slug.ilike(pattern));
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

    pub async fn list_catalog_slugs(&self) -> Result<Vec<String>, RepositoryError> {
        self.run_blocking(move |conn| {
            use crate::db::schema::items::dsl::*;

            items.select(slug).load::<String>(conn).map_err(Into::into)
        })
        .await
    }
}
