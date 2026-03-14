use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::upsert::excluded;

use crate::{
    db::Database,
    models::mob::MobModel,
    repositories::{Repository, RepositoryError},
};

#[derive(Clone)]
pub struct PgMobRepository {
    db: Database,
}

impl Repository for PgMobRepository {
    fn db(&self) -> Database {
        self.db.clone()
    }
}

impl PgMobRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn upsert_catalog_mob(
        &self,
        mob_slug: &'static str,
    ) -> Result<MobModel, RepositoryError> {
        self.run_blocking(move |conn| {
            use crate::db::schema::mobs::dsl::*;

            let model = MobModel::new(mob_slug.to_string());

            insert_into(mobs)
                .values(&model)
                .on_conflict(slug)
                .do_update()
                .set(slug.eq(excluded(slug)))
                .get_result::<MobModel>(conn)
                .map_err(Into::into)
        })
        .await
    }

    pub async fn list_catalog_slugs(&self) -> Result<Vec<String>, RepositoryError> {
        self.run_blocking(move |conn| {
            use crate::db::schema::mobs::dsl::*;

            mobs.select(slug).load::<String>(conn).map_err(Into::into)
        })
        .await
    }
}
