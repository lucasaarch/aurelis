use crate::{
    db::{Database, schema::mobs},
    models::mob::MobModel,
    repositories::{Repository, RepositoryError},
};
use diesel::prelude::*;
use shared::models::mob::Mob;

pub struct CreateMobParams {
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub mob_type: String,
}

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

    pub async fn create(&self, params: CreateMobParams) -> Result<Mob, RepositoryError> {
        let model = MobModel::new(
            params.slug,
            params.name,
            params.description,
            params
                .mob_type
                .parse()
                .map_err(|_| RepositoryError::NotFound)?,
        );

        self.run_blocking(move |conn| {
            diesel::insert_into(mobs::table)
                .values(&model)
                .get_result(conn)
                .map(|m: MobModel| m.into())
                .map_err(Into::into)
        })
        .await
    }
}
