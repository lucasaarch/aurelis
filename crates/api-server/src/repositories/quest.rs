use diesel::dsl::insert_into;
use diesel::prelude::*;
use diesel::upsert::excluded;

use crate::{
    db::Database,
    models::quest::QuestModel,
    repositories::{Repository, RepositoryError},
};

#[derive(Clone)]
pub struct PgQuestRepository {
    db: Database,
}

impl Repository for PgQuestRepository {
    fn db(&self) -> Database {
        self.db.clone()
    }
}

impl PgQuestRepository {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn upsert_catalog_quest(
        &self,
        quest_slug: &'static str,
    ) -> Result<QuestModel, RepositoryError> {
        self.run_blocking(move |conn| {
            use crate::db::schema::quests::dsl::*;

            let model = QuestModel::new(quest_slug.to_string());

            insert_into(quests)
                .values(&model)
                .on_conflict(slug)
                .do_update()
                .set(slug.eq(excluded(slug)))
                .get_result::<QuestModel>(conn)
                .map_err(Into::into)
        })
        .await
    }

    pub async fn list_catalog_slugs(&self) -> Result<Vec<String>, RepositoryError> {
        self.run_blocking(move |conn| {
            use crate::db::schema::quests::dsl::*;

            quests.select(slug).load::<String>(conn).map_err(Into::into)
        })
        .await
    }
}
