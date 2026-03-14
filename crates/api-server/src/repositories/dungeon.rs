use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::upsert::excluded;

use crate::{
    db::Database,
    models::dungeon::DungeonModel,
    repositories::{Repository, RepositoryError},
};

#[derive(Clone)]
pub struct PgDungeonRepository {
    db: Database,
}

impl Repository for PgDungeonRepository {
    fn db(&self) -> Database {
        self.db.clone()
    }
}

impl PgDungeonRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn upsert_catalog_dungeon(
        &self,
        dungeon_slug: &'static str,
    ) -> Result<DungeonModel, RepositoryError> {
        self.run_blocking(move |conn| {
            use crate::db::schema::dungeons::dsl::*;

            let model = DungeonModel::new(dungeon_slug.to_string());

            insert_into(dungeons)
                .values(&model)
                .on_conflict(slug)
                .do_update()
                .set(slug.eq(excluded(slug)))
                .get_result::<DungeonModel>(conn)
                .map_err(Into::into)
        })
        .await
    }

    pub async fn list_catalog_slugs(&self) -> Result<Vec<String>, RepositoryError> {
        self.run_blocking(move |conn| {
            use crate::db::schema::dungeons::dsl::*;

            dungeons
                .select(slug)
                .load::<String>(conn)
                .map_err(Into::into)
        })
        .await
    }
}
