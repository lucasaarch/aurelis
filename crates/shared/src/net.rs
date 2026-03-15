use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::{
    character_data::CombatAffinity,
    combat_stats::{CombatStats, FixedStatLine},
    reward_stats::RewardStats,
};

pub const RELIABLE_GAME_CHANNEL_ID: u8 = 0;

pub fn protocol_id_from_version(version: &str) -> u64 {
    let (core, prerelease) = version.split_once('-').unwrap_or((version, ""));
    let mut parts = core.split('.');

    let major = parts
        .next()
        .and_then(|value| value.parse::<u64>().ok())
        .expect("invalid major version for protocol id");
    let minor = parts
        .next()
        .and_then(|value| value.parse::<u64>().ok())
        .expect("invalid minor version for protocol id");
    let patch = parts
        .next()
        .and_then(|value| value.parse::<u64>().ok())
        .expect("invalid patch version for protocol id");

    let prerelease_iteration = prerelease
        .rsplit('.')
        .next()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(0);

    (major << 48) | (minor << 32) | (patch << 16) | prerelease_iteration
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveBuffState {
    pub effect_slug: String,
    pub remaining_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillCooldownState {
    pub skill_slug: String,
    pub remaining_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterStatsSnapshot {
    pub final_combat_stats: CombatStats,
    pub final_reward_stats: RewardStats,
    pub base_combat_stats: CombatStats,
    pub class_combat_stats: CombatStats,
    pub equipment_combat_stats: CombatStats,
    pub persistent_combat_stats: CombatStats,
    pub timed_combat_stats: CombatStats,
    pub base_reward_stats: RewardStats,
    pub class_reward_stats: RewardStats,
    pub equipment_reward_stats: RewardStats,
    pub persistent_reward_stats: RewardStats,
    pub timed_reward_stats: RewardStats,
    pub current_hp: i32,
    pub current_mp: i32,
    pub active_buffs: Vec<ActiveBuffState>,
    pub skill_cooldowns: Vec<SkillCooldownState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSkillView {
    pub slug: String,
    pub name: String,
    pub description: String,
    pub kind: String,
    pub mp_cost: i32,
    pub cooldown_ms: u64,
    pub cast_time_ms: u64,
    pub range: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemModifierView {
    pub id: String,
    pub stat: String,
    pub kind: String,
    pub value: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedItemView {
    pub item_instance_id: Option<Uuid>,
    pub item_slug: String,
    pub name: String,
    pub description: String,
    pub inventory_type: String,
    pub rarity: String,
    pub equipment_slot: Option<String>,
    pub quantity: i16,
    pub refinement: i16,
    pub base_gem_slots: i16,
    pub bonus_gem_slots: i16,
    pub fixed_stats: Vec<FixedStatLine>,
    pub fixed_special_effects: Vec<ItemModifierView>,
    pub additional_effects: Vec<ItemModifierView>,
    pub socketed_gems: Vec<ResolvedItemView>,
    pub resolved_combat_stats: CombatStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventorySlotView {
    pub slot_index: i16,
    pub quantity: i16,
    pub item: Option<ResolvedItemView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryView {
    pub inventory_id: Uuid,
    pub inventory_type: String,
    pub capacity: i16,
    pub slots: Vec<InventorySlotView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquippedSlotView {
    pub slot: String,
    pub item: ResolvedItemView,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSnapshotView {
    pub character_id: Uuid,
    pub name: String,
    pub base_character_slug: String,
    pub current_class_slug: String,
    pub level: i16,
    pub experience: i64,
    pub credits: i64,
    pub affinity: CombatAffinity,
    pub available_skills: Vec<CharacterSkillView>,
    pub stats: CharacterStatsSnapshot,
    pub inventories: Vec<InventoryView>,
    pub equipped: Vec<EquippedSlotView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSummaryView {
    pub character_id: Uuid,
    pub name: String,
    pub level: i16,
    pub class_slug: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    Login {
        email: String,
        password: String,
    },
    Authenticate {
        token: String,
    },
    ListCharacters,
    CreateCharacter {
        name: String,
        class_slug: String,
    },
    SelectCharacter {
        character_id: Uuid,
    },
    UseItem {
        inventory_type: String,
        slot: i16,
    },
    UseItemOnEquipment {
        inventory_type: String,
        slot: i16,
        equipment_slot: String,
    },
    UseSkill {
        skill_slug: String,
    },
    EquipItem {
        inventory_type: String,
        slot: i16,
    },
    UnequipItem {
        equipment_slot: String,
    },
    RefineEquipment {
        equipment_slot: String,
    },
    SocketGem {
        equipment_slot: String,
        inventory_type: String,
        slot: i16,
        socket_index: i16,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    LoginSucceeded {
        account_id: Uuid,
    },
    LoginFailed {
        reason: String,
    },
    Authenticated {
        account_id: Uuid,
    },
    AuthenticationFailed {
        reason: String,
    },
    CharacterSelected {
        character_id: Uuid,
    },
    CharactersListed {
        characters: Vec<CharacterSummaryView>,
    },
    CharacterListFailed {
        reason: String,
    },
    CharacterCreated {
        character: CharacterSummaryView,
    },
    CharacterCreationFailed {
        reason: String,
    },
    CharacterSelectionFailed {
        reason: String,
    },
    ItemUsed {
        inventory_type: String,
        slot: i16,
    },
    ItemUsedOnEquipment {
        inventory_type: String,
        slot: i16,
        equipment_slot: String,
    },
    ItemUseFailed {
        reason: String,
    },
    SkillUsed {
        skill_slug: String,
    },
    SkillUseFailed {
        reason: String,
    },
    ItemEquipped {
        inventory_type: String,
        slot: i16,
        equipment_slot: String,
    },
    ItemUnequipped {
        equipment_slot: String,
    },
    EquipmentChangeFailed {
        reason: String,
    },
    EquipmentRefined {
        equipment_slot: String,
        old_refinement: i16,
        new_refinement: i16,
        outcome: String,
    },
    EquipmentRefineFailed {
        reason: String,
    },
    GemSocketed {
        equipment_slot: String,
        socket_index: i16,
    },
    GemSocketFailed {
        reason: String,
    },
    RuntimeStateUpdated {
        current_hp: i32,
        current_mp: i32,
        active_buffs: Vec<ActiveBuffState>,
        skill_cooldowns: Vec<SkillCooldownState>,
    },
    CharacterSnapshotLoaded {
        snapshot: CharacterSnapshotView,
    },
    CharacterStatsUpdated {
        stats: CharacterStatsSnapshot,
    },
    CharacterInventoryUpdated {
        inventories: Vec<InventoryView>,
        equipped: Vec<EquippedSlotView>,
    },
}
