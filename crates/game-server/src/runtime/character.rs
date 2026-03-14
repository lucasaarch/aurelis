use std::collections::HashMap;

use shared::models::{
    combat_stats::CombatStats, equipment_slot::EquipmentSlot, item_data::ItemData,
};
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Clone)]
pub struct RuntimeCharacter {
    pub account_id: Uuid,
    pub character_id: Uuid,
    pub name: String,
    pub base_character_slug: String,
    pub current_class_slug: String,
    pub level: i16,
    pub experience: i64,
    pub credits: i64,
    pub loadout: RuntimeLoadout,
    pub stats: RuntimeStatBlock,
}

#[allow(dead_code)]
#[derive(Clone, Default)]
pub struct RuntimeLoadout {
    pub equipped: HashMap<EquipmentSlot, ResolvedEquippedItem>,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct ResolvedEquippedItem {
    pub item_instance_id: Uuid,
    pub item_slug: String,
    pub item_data: &'static ItemData,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct RuntimeStatBlock {
    pub base: CombatStats,
    pub from_class: CombatStats,
    pub from_equipment: CombatStats,
    pub final_stats: CombatStats,
}
