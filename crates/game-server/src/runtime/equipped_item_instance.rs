use std::collections::HashSet;

use shared::{
    data::cities::find_item_by_slug,
    models::{
        combat_stats::{CombatStats, FixedStatLine, StatKey},
        equipment_slot::EquipmentSlot,
        item_data::{CatalogStatModifier, ItemData},
        item_instance_attributes::{ItemInstanceStatModifier, StatModifierValueKind},
        stat_modifier::ModifierStat,
    },
};

use crate::{
    resources::internal_api::{PersistedItemInstance, PlayableCharacterSnapshot},
    runtime::item_instance_rolls::parse_attributes,
    runtime::modifier::{ModifierDuration, ModifierSource, RuntimeModifier, StatModifierOp},
};

pub struct EquippedItemInstanceCalculation {
    pub flat_stats: CombatStats,
    pub modifiers: Vec<RuntimeModifier>,
}

pub fn calculate_equipped_item_instance(
    snapshot: &PlayableCharacterSnapshot,
    item_instance: &PersistedItemInstance,
    item_data: &'static ItemData,
) -> Result<EquippedItemInstanceCalculation, String> {
    let fixed_stats = item_data
        .kind
        .fixed_stats()
        .ok_or_else(|| format!("item '{}' is missing fixed stats", item_data.slug))?;

    let mut flat_stats = CombatStats::ZERO;
    flat_stats.add_lines(fixed_stats);

    let attributes = parse_attributes(&item_instance.attributes_json)?;
    let mut refinable_stats = CombatStats::ZERO;
    refinable_stats.add_lines(fixed_stats);

    let mut modifiers = Vec::new();
    for effect in &attributes.additional_effects {
        match split_instance_modifier(effect) {
            SplitModifier::FlatCombat(line) => {
                flat_stats.add_line(line);
            }
            SplitModifier::Runtime(op) => modifiers.push(RuntimeModifier {
                source: ModifierSource::Equipment {
                    item_instance_id: item_instance.id,
                },
                duration: ModifierDuration::Permanent,
                operations: vec![op],
            }),
        }
    }

    let gem_stats = calculate_gem_stats(snapshot, item_instance, item_data)?;
    flat_stats += gem_stats.flat_stats;
    modifiers.extend(gem_stats.modifiers);

    let equipment_slot = item_data
        .kind
        .equipment_slot()
        .ok_or_else(|| format!("item '{}' is not equippable", item_data.slug))?;
    flat_stats += refinement_bonus(equipment_slot, item_instance.refinement, &refinable_stats);

    Ok(EquippedItemInstanceCalculation {
        flat_stats,
        modifiers,
    })
}

struct GemCalculation {
    flat_stats: CombatStats,
    modifiers: Vec<RuntimeModifier>,
}

fn calculate_gem_stats(
    snapshot: &PlayableCharacterSnapshot,
    item_instance: &PersistedItemInstance,
    item_data: &'static ItemData,
) -> Result<GemCalculation, String> {
    let slot_config = item_data
        .kind
        .gem_slots()
        .ok_or_else(|| format!("item '{}' does not support gem slots", item_data.slug))?;
    let bonus_slots = item_instance
        .bonus_gem_slots
        .clamp(0, slot_config.max_bonus_slots);
    let total_slots = slot_config.base_slots + bonus_slots;
    if item_instance.gems.len() > total_slots as usize {
        return Err(format!(
            "item instance '{}' has more gems than available slots",
            item_instance.id
        ));
    }

    let mut seen_slots = HashSet::new();
    let mut flat_stats = CombatStats::ZERO;
    let mut modifiers = Vec::new();

    for socketed_gem in &item_instance.gems {
        if socketed_gem.slot_index < 0 || socketed_gem.slot_index >= total_slots {
            return Err(format!(
                "gem socket index '{}' is invalid for item instance '{}'",
                socketed_gem.slot_index, item_instance.id
            ));
        }
        if !seen_slots.insert(socketed_gem.slot_index) {
            return Err(format!(
                "duplicate gem socket index '{}' for item instance '{}'",
                socketed_gem.slot_index, item_instance.id
            ));
        }

        let gem_instance = snapshot
            .item_instances
            .iter()
            .find(|candidate| candidate.id == socketed_gem.gem_instance_id)
            .ok_or_else(|| {
                format!(
                    "missing gem item instance '{}' for equipment '{}'",
                    socketed_gem.gem_instance_id, item_instance.id
                )
            })?;
        let gem_item_data = find_item_by_slug(&gem_instance.item_slug).ok_or_else(|| {
            format!(
                "unknown gem item slug '{}' for item instance '{}'",
                gem_instance.item_slug, gem_instance.id
            )
        })?;
        let fixed_gem_modifiers = gem_item_data
            .kind
            .gem_modifiers()
            .ok_or_else(|| format!("item '{}' is not a valid gem", gem_item_data.slug))?;

        for modifier in fixed_gem_modifiers {
            ensure_valid_gem_modifier(modifier, gem_item_data)?;
            match split_catalog_modifier(modifier) {
                SplitModifier::FlatCombat(line) => {
                    flat_stats.add_line(line);
                }
                SplitModifier::Runtime(op) => modifiers.push(RuntimeModifier {
                    source: ModifierSource::Gem {
                        item_instance_id: item_instance.id,
                        slot_index: socketed_gem.slot_index,
                    },
                    duration: ModifierDuration::Permanent,
                    operations: vec![op],
                }),
            }
        }

        let gem_attributes = parse_attributes(&gem_instance.attributes_json)?;
        for modifier in &gem_attributes.additional_effects {
            ensure_valid_instance_gem_modifier(modifier, gem_item_data)?;
            match split_instance_modifier(modifier) {
                SplitModifier::FlatCombat(line) => {
                    flat_stats.add_line(line);
                }
                SplitModifier::Runtime(op) => modifiers.push(RuntimeModifier {
                    source: ModifierSource::Gem {
                        item_instance_id: item_instance.id,
                        slot_index: socketed_gem.slot_index,
                    },
                    duration: ModifierDuration::Permanent,
                    operations: vec![op],
                }),
            }
        }
    }

    Ok(GemCalculation {
        flat_stats,
        modifiers,
    })
}

fn ensure_valid_gem_modifier(
    modifier: &CatalogStatModifier,
    gem_item_data: &ItemData,
) -> Result<(), String> {
    match modifier.stat {
        ModifierStat::Combat(StatKey::PhysicalAtk)
        | ModifierStat::Combat(StatKey::MagicalAtk)
        | ModifierStat::Combat(StatKey::PhysicalDef)
        | ModifierStat::Combat(StatKey::MagicalDef) => Err(format!(
            "gem '{}' uses forbidden stat '{:?}'",
            gem_item_data.slug, modifier.stat
        )),
        _ => Ok(()),
    }
}

fn ensure_valid_instance_gem_modifier(
    modifier: &ItemInstanceStatModifier,
    gem_item_data: &ItemData,
) -> Result<(), String> {
    match modifier.stat {
        ModifierStat::Combat(StatKey::PhysicalAtk)
        | ModifierStat::Combat(StatKey::MagicalAtk)
        | ModifierStat::Combat(StatKey::PhysicalDef)
        | ModifierStat::Combat(StatKey::MagicalDef) => Err(format!(
            "gem '{}' rolled forbidden stat '{:?}'",
            gem_item_data.slug, modifier.stat
        )),
        _ => Ok(()),
    }
}

enum SplitModifier {
    FlatCombat(FixedStatLine),
    Runtime(StatModifierOp),
}

fn split_instance_modifier(effect: &ItemInstanceStatModifier) -> SplitModifier {
    split_modifier(effect.stat, effect.kind, effect.value)
}

fn split_catalog_modifier(effect: &CatalogStatModifier) -> SplitModifier {
    split_modifier(effect.stat, effect.kind, effect.value)
}

fn split_modifier(stat: ModifierStat, kind: StatModifierValueKind, value: i32) -> SplitModifier {
    match (stat, kind) {
        (ModifierStat::Combat(stat), StatModifierValueKind::Flat) => {
            SplitModifier::FlatCombat(FixedStatLine { stat, value })
        }
        (stat, StatModifierValueKind::Flat) => {
            SplitModifier::Runtime(StatModifierOp::AddFlat { stat, value })
        }
        (stat, StatModifierValueKind::Percent) => {
            SplitModifier::Runtime(StatModifierOp::AddPercent {
                stat,
                value_bp: value,
            })
        }
    }
}

fn refinement_bonus(slot: EquipmentSlot, refinement: i16, base_stats: &CombatStats) -> CombatStats {
    slot.calculate_refinement_bonus(refinement, base_stats)
}
