use crate::models::{
    character_data::CombatAffinity,
    combat_stats::StatKey,
    item_data::CatalogStatModifier,
    item_instance_attributes::StatModifierValueKind,
    skill_data::{
        CharacterSkillUnlockTier, SkillCost, SkillData, SkillKind, SkillOwner, SkillTimedBuffData,
        SkillUnlockRequirement, SpecialActiveTier,
    },
    stat_modifier::ModifierStat,
};

const NO_SKILL_MODIFIERS: &[CatalogStatModifier] = &[];

const SENTINEL_STEEL_PULSE_BUFF: &[CatalogStatModifier] = &[CatalogStatModifier {
    id: "steel_pulse_physical_atk_pct",
    stat: ModifierStat::Combat(StatKey::PhysicalAtk),
    kind: StatModifierValueKind::Percent,
    value: 1500,
}];

const SENTINEL_DISCIPLINE_PASSIVE: &[CatalogStatModifier] = &[
    CatalogStatModifier {
        id: "sentinel_discipline_mp",
        stat: ModifierStat::Combat(StatKey::Mp),
        kind: StatModifierValueKind::Flat,
        value: 100,
    },
    CatalogStatModifier {
        id: "sentinel_discipline_crit",
        stat: ModifierStat::Combat(StatKey::CritChance),
        kind: StatModifierValueKind::Flat,
        value: 8,
    },
];

pub static SENTINEL_STEEL_PULSE: SkillData = SkillData {
    slug: "sentinel_steel_pulse",
    name: "Steel Pulse",
    description: "A disciplined pulse of force that fortifies Kael and disrupts nearby enemies.",
    owner: SkillOwner::Class {
        class_slug: "kael_royal_sentinel",
    },
    kind: SkillKind::Advantage,
    scaling_affinity: CombatAffinity::Physical,
    unlock_requirement: SkillUnlockRequirement::Level { required_level: 10 },
    cost: SkillCost::Mp(36),
    cooldown_secs: 12.0,
    cast_time_secs: 0.6,
    range: 3.5,
    passive_modifiers: NO_SKILL_MODIFIERS,
    timed_buff: Some(SkillTimedBuffData {
        duration_ms: 15_000,
        modifiers: SENTINEL_STEEL_PULSE_BUFF,
    }),
};

pub static SENTINEL_DISCIPLINE: SkillData = SkillData {
    slug: "sentinel_discipline",
    name: "Sentinel Discipline",
    description: "A passive discipline that deepens Kael's reserves and sharpens his critical instinct.",
    owner: SkillOwner::Class {
        class_slug: "kael_royal_sentinel",
    },
    kind: SkillKind::Passive,
    scaling_affinity: CombatAffinity::Physical,
    unlock_requirement: SkillUnlockRequirement::Level { required_level: 20 },
    cost: SkillCost::None,
    cooldown_secs: 0.0,
    cast_time_secs: 0.0,
    range: 0.0,
    passive_modifiers: SENTINEL_DISCIPLINE_PASSIVE,
    timed_buff: None,
};

pub static SENTINEL_SIGNATURE_DECREE: SkillData = SkillData {
    slug: "sentinel_signature_decree",
    name: "Signature Decree",
    description: "Kael invokes a royal command that surges through his blade and empowers his stance.",
    owner: SkillOwner::Class {
        class_slug: "kael_royal_sentinel",
    },
    kind: SkillKind::SpecialActive(SpecialActiveTier::Signature),
    scaling_affinity: CombatAffinity::Physical,
    unlock_requirement: SkillUnlockRequirement::CharacterBookTier {
        required_level: 15,
        tier: CharacterSkillUnlockTier::Beginner,
    },
    cost: SkillCost::Mp(100),
    cooldown_secs: 28.0,
    cast_time_secs: 0.8,
    range: 4.0,
    passive_modifiers: NO_SKILL_MODIFIERS,
    timed_buff: None,
};
