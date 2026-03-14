use crate::models::mob_type::MobType;

pub struct MobData {
    pub slug: &'static str,
    pub name: &'static str,
    pub level: i32,
    pub hp: i32,
    pub mp: i32,
    pub physical_atk: i32,
    pub magical_atk: i32,
    pub physical_def: i32,
    pub magical_def: i32,
    pub experience_reward: i32,
    pub credits_reward: i32,
    pub move_spd: f32,
    pub atk_spd: f32,
    pub aggro_range: i32,
    pub leash_range: i32,
    pub crit_resistance: f32,
    pub knockback_resistance: f32,
    pub cc_resistance: f32,
    pub mob_type: MobType,
}
