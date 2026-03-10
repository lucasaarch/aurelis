use diesel::prelude::{Insertable, Queryable};
use shared::models::dungeon_mob::DungeonMob;
use uuid::Uuid;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::dungeon_mobs)]
pub struct DungeonMobModel {
    pub id: Uuid,
    pub mob_id: Uuid,
    pub dungeon_id: String,
}

impl DungeonMobModel {
    pub fn new(mob_id: Uuid, dungeon_id: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            mob_id,
            dungeon_id,
        }
    }
}

impl From<DungeonMobModel> for DungeonMob {
    fn from(model: DungeonMobModel) -> Self {
        Self {
            id: model.id,
            mob_id: model.mob_id,
            dungeon_id: model.dungeon_id,
        }
    }
}
