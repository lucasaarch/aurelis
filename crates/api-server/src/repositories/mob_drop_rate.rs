use crate::{
    db::schema::mob_drop_rates,
    models::mob_drop_rate::MobDropRateModel,
    repositories::{Repository, RepositoryError},
};
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use shared::models::mob_drop_rate::MobDropRate;
use uuid::Uuid;

pub struct CreateMobDropRateParams {
    pub mob_id: Uuid,
    pub item_id: Uuid,
    pub drop_chance: BigDecimal,
}

#[derive(Clone)]
pub struct PgMobDropRateRepository {
    db: crate::db::Database,
}

impl Repository for PgMobDropRateRepository {
    fn db(&self) -> crate::db::Database {
        self.db.clone()
    }
}

impl PgMobDropRateRepository {
    pub fn new(db: crate::db::Database) -> Self {
        Self { db }
    }

    pub async fn create(
        &self,
        params: CreateMobDropRateParams,
    ) -> Result<MobDropRate, RepositoryError> {
        let model = MobDropRateModel::new(params.mob_id, params.item_id, params.drop_chance);

        self.run_blocking(move |conn| {
            diesel::insert_into(mob_drop_rates::table)
                .values(&model)
                .get_result::<MobDropRateModel>(conn)
                .map(|m| m.into())
                .map_err(Into::into)
        })
        .await
    }
}
