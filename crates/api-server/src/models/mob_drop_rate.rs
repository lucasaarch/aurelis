use bigdecimal::BigDecimal;
use diesel::prelude::{Insertable, Queryable};
use shared::models::mob_drop_rate::MobDropRate;
use uuid::Uuid;

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::db::schema::mob_drop_rates)]
pub struct MobDropRateModel {
    pub id: Uuid,
    pub mob_id: Uuid,
    pub item_id: Uuid,
    pub drop_chance: BigDecimal,
}

impl MobDropRateModel {
    pub fn new(mob_id: Uuid, item_id: Uuid, drop_chance: BigDecimal) -> Self {
        Self {
            id: Uuid::new_v4(),
            mob_id,
            item_id,
            drop_chance,
        }
    }
}

impl From<MobDropRateModel> for MobDropRate {
    fn from(model: MobDropRateModel) -> Self {
        Self {
            id: model.id,
            mob_id: model.mob_id,
            item_id: model.item_id,
            drop_chance: model.drop_chance,
        }
    }
}
