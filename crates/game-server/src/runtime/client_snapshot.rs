use shared::{
    data::{
        characters::{all_skills_for_character, find_character_by_slug},
        cities::find_item_by_slug,
    },
    models::{
        combat_stats::CombatStats,
        item_data::{CatalogStatModifier, ItemData},
        item_instance_attributes::{ItemInstanceAttributes, ItemInstanceStatModifier},
        skill_data::{SkillCost, SkillData, SkillKind, SpecialActiveTier},
    },
    net::{
        CharacterSkillView, CharacterSnapshotView, CharacterStatsSnapshot, EquippedSlotView,
        InventorySlotView, InventoryView, ItemModifierView, ResolvedItemView,
    },
};

use crate::{
    resources::internal_api::{
        PersistedInventory, PersistedInventoryItem, PersistedItemInstance,
        PlayableCharacterSnapshot,
    },
    runtime::{
        character::RuntimeCharacter, equipped_item_instance::calculate_equipped_item_instance,
        item_instance_rolls::parse_attributes,
    },
};

pub fn build_character_snapshot_view(
    runtime: &RuntimeCharacter,
    snapshot: &PlayableCharacterSnapshot,
) -> Result<CharacterSnapshotView, String> {
    Ok(CharacterSnapshotView {
        character_id: runtime.character_id,
        name: runtime.name.clone(),
        base_character_slug: runtime.base_character_slug.clone(),
        current_class_slug: runtime.current_class_slug.clone(),
        level: runtime.level,
        experience: runtime.experience,
        credits: runtime.credits,
        affinity: runtime.combat_affinity,
        available_skills: build_skill_views(runtime)?,
        stats: build_character_stats_snapshot(runtime),
        inventories: snapshot
            .inventories
            .iter()
            .map(|inventory| build_inventory_view(snapshot, inventory))
            .collect::<Result<Vec<_>, _>>()?,
        equipped: build_equipped_views(snapshot, runtime)?,
    })
}

pub fn build_character_stats_snapshot(runtime: &RuntimeCharacter) -> CharacterStatsSnapshot {
    CharacterStatsSnapshot {
        final_combat_stats: runtime.stats.final_stats,
        final_reward_stats: runtime.rewards.final_stats,
        base_combat_stats: runtime.stats.base,
        class_combat_stats: runtime.stats.from_class,
        equipment_combat_stats: runtime.stats.from_equipment,
        persistent_combat_stats: runtime.stats.from_persistent_modifiers,
        timed_combat_stats: runtime.stats.from_timed_modifiers,
        base_reward_stats: runtime.rewards.base,
        class_reward_stats: runtime.rewards.from_class,
        equipment_reward_stats: runtime.rewards.from_equipment,
        persistent_reward_stats: runtime.rewards.from_persistent_modifiers,
        timed_reward_stats: runtime.rewards.from_timed_modifiers,
        current_hp: runtime.resources.current_hp,
        current_mp: runtime.resources.current_mp,
        active_buffs: runtime.active_buffs(),
        skill_cooldowns: runtime.skill_cooldowns(),
    }
}

pub fn build_inventory_views(
    snapshot: &PlayableCharacterSnapshot,
) -> Result<Vec<InventoryView>, String> {
    snapshot
        .inventories
        .iter()
        .map(|inventory| build_inventory_view(snapshot, inventory))
        .collect()
}

pub fn build_equipped_views(
    snapshot: &PlayableCharacterSnapshot,
    runtime: &RuntimeCharacter,
) -> Result<Vec<EquippedSlotView>, String> {
    let mut equipped = runtime
        .loadout
        .equipped
        .iter()
        .map(|(slot, resolved)| {
            let item_instance = snapshot
                .item_instances
                .iter()
                .find(|candidate| candidate.id == resolved.item_instance_id)
                .ok_or_else(|| {
                    format!(
                        "missing equipped item instance '{}' for slot '{slot:?}'",
                        resolved.item_instance_id
                    )
                })?;
            Ok(EquippedSlotView {
                slot: format!("{slot:?}").to_lowercase(),
                item: build_item_view_from_instance(snapshot, item_instance, 1)?,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    equipped.sort_by(|a, b| a.slot.cmp(&b.slot));
    Ok(equipped)
}

fn build_inventory_view(
    snapshot: &PlayableCharacterSnapshot,
    inventory: &PersistedInventory,
) -> Result<InventoryView, String> {
    let mut slots = inventory
        .items
        .iter()
        .map(|item| build_inventory_slot_view(snapshot, item))
        .collect::<Result<Vec<_>, _>>()?;
    slots.sort_by_key(|slot| slot.slot_index);
    Ok(InventoryView {
        inventory_id: inventory.id,
        inventory_type: inventory.inventory_type.clone(),
        capacity: inventory.capacity,
        slots,
    })
}

fn build_inventory_slot_view(
    snapshot: &PlayableCharacterSnapshot,
    item: &PersistedInventoryItem,
) -> Result<InventorySlotView, String> {
    let resolved_item = if let Some(item_instance_id) = item.item_instance_id {
        let item_instance = snapshot
            .item_instances
            .iter()
            .find(|candidate| candidate.id == item_instance_id)
            .ok_or_else(|| format!("missing item instance '{item_instance_id}'"))?;
        Some(build_item_view_from_instance(
            snapshot,
            item_instance,
            item.quantity,
        )?)
    } else if let Some(item_slug) = item.item_slug.as_deref() {
        let item_data = find_item_by_slug(item_slug)
            .ok_or_else(|| format!("unknown item slug '{item_slug}'"))?;
        Some(build_catalog_item_view(item_data, item.quantity))
    } else {
        None
    };

    Ok(InventorySlotView {
        slot_index: item.slot_index,
        quantity: item.quantity,
        item: resolved_item,
    })
}

fn build_item_view_from_instance(
    snapshot: &PlayableCharacterSnapshot,
    item_instance: &PersistedItemInstance,
    quantity: i16,
) -> Result<ResolvedItemView, String> {
    let item_data = find_item_by_slug(&item_instance.item_slug)
        .ok_or_else(|| format!("unknown item slug '{}'", item_instance.item_slug))?;
    let attributes = parse_attributes(&item_instance.attributes_json)?;
    let calculated = if item_data.kind.equipment_slot().is_some() {
        calculate_equipped_item_instance(snapshot, item_instance, item_data)?.flat_stats
    } else {
        resolve_non_equipment_instance_stats(&attributes, item_data)
    };

    let socketed_gems = item_instance
        .gems
        .iter()
        .map(|gem| {
            let gem_instance = snapshot
                .item_instances
                .iter()
                .find(|candidate| candidate.id == gem.gem_instance_id)
                .ok_or_else(|| format!("missing gem item instance '{}'", gem.gem_instance_id))?;
            build_item_view_from_instance(snapshot, gem_instance, 1)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(ResolvedItemView {
        item_instance_id: Some(item_instance.id),
        item_slug: item_data.slug.to_string(),
        name: item_data.name.to_string(),
        description: item_data.description.to_string(),
        inventory_type: item_instance.inventory_type.clone(),
        rarity: format!("{:?}", item_data.rarity).to_lowercase(),
        equipment_slot: item_data
            .kind
            .equipment_slot()
            .map(|slot| format!("{slot:?}").to_lowercase()),
        quantity,
        refinement: item_instance.refinement,
        base_gem_slots: item_data
            .kind
            .gem_slots()
            .map(|slots| slots.base_slots)
            .unwrap_or(0),
        bonus_gem_slots: item_instance.bonus_gem_slots,
        fixed_stats: item_data.kind.fixed_stats().unwrap_or_default().to_vec(),
        fixed_special_effects: map_catalog_modifiers(
            item_data.kind.fixed_special_effects().unwrap_or_default(),
        ),
        additional_effects: map_instance_modifiers(&attributes.additional_effects),
        socketed_gems,
        resolved_combat_stats: calculated,
    })
}

fn build_catalog_item_view(item_data: &'static ItemData, quantity: i16) -> ResolvedItemView {
    ResolvedItemView {
        item_instance_id: None,
        item_slug: item_data.slug.to_string(),
        name: item_data.name.to_string(),
        description: item_data.description.to_string(),
        inventory_type: format!("{:?}", item_data.inventory_type).to_lowercase(),
        rarity: format!("{:?}", item_data.rarity).to_lowercase(),
        equipment_slot: item_data
            .kind
            .equipment_slot()
            .map(|slot| format!("{slot:?}").to_lowercase()),
        quantity,
        refinement: 0,
        base_gem_slots: item_data
            .kind
            .gem_slots()
            .map(|slots| slots.base_slots)
            .unwrap_or(0),
        bonus_gem_slots: 0,
        fixed_stats: item_data.kind.fixed_stats().unwrap_or_default().to_vec(),
        fixed_special_effects: map_catalog_modifiers(
            item_data.kind.fixed_special_effects().unwrap_or_default(),
        ),
        additional_effects: vec![],
        socketed_gems: vec![],
        resolved_combat_stats: resolve_catalog_item_stats(item_data),
    }
}

fn resolve_non_equipment_instance_stats(
    attributes: &ItemInstanceAttributes,
    item_data: &ItemData,
) -> CombatStats {
    let mut result = resolve_catalog_item_stats(item_data);
    for modifier in &attributes.additional_effects {
        if let Some(line) = instance_modifier_as_flat_line(modifier) {
            result.add_line(line);
        }
    }
    result
}

fn resolve_catalog_item_stats(item_data: &ItemData) -> CombatStats {
    let mut result = CombatStats::ZERO;
    result.add_lines(item_data.kind.fixed_stats().unwrap_or_default());
    if let Some(gem_modifiers) = item_data.kind.gem_modifiers() {
        for modifier in gem_modifiers {
            if let Some(line) = catalog_modifier_as_flat_line(modifier) {
                result.add_line(line);
            }
        }
    }
    result
}

fn build_skill_views(runtime: &RuntimeCharacter) -> Result<Vec<CharacterSkillView>, String> {
    let character = find_character_by_slug(&runtime.base_character_slug).ok_or_else(|| {
        format!(
            "unknown base character slug '{}'",
            runtime.base_character_slug
        )
    })?;
    let mut result = all_skills_for_character(&runtime.base_character_slug)
        .into_iter()
        .filter(|skill| {
            runtime
                .available_skill_slugs
                .iter()
                .any(|slug| slug == skill.slug)
        })
        .filter(|skill| {
            skill.is_unlocked_for(
                character,
                &runtime.current_class_slug,
                runtime.level,
                &runtime.skill_unlocks,
            )
        })
        .map(map_skill_view)
        .collect::<Vec<_>>();
    result.sort_by(|a, b| a.slug.cmp(&b.slug));
    Ok(result)
}

fn map_skill_view(skill: &SkillData) -> CharacterSkillView {
    CharacterSkillView {
        slug: skill.slug.to_string(),
        name: skill.name.to_string(),
        description: skill.description.to_string(),
        kind: match skill.kind {
            SkillKind::Active => "active".to_string(),
            SkillKind::Advantage => "advantage".to_string(),
            SkillKind::Passive => "passive".to_string(),
            SkillKind::SpecialActive(tier) => match tier {
                SpecialActiveTier::Signature => "special_signature".to_string(),
                SpecialActiveTier::Awakened => "special_awakened".to_string(),
                SpecialActiveTier::Ascendant => "special_ascendant".to_string(),
            },
        },
        mp_cost: match skill.cost {
            SkillCost::None => 0,
            SkillCost::Mp(value) => value,
        },
        cooldown_ms: (skill.cooldown_secs * 1000.0).round() as u64,
        cast_time_ms: (skill.cast_time_secs * 1000.0).round() as u64,
        range: skill.range,
    }
}

fn map_catalog_modifiers(modifiers: &[CatalogStatModifier]) -> Vec<ItemModifierView> {
    modifiers
        .iter()
        .map(|modifier| ItemModifierView {
            id: modifier.id.to_string(),
            stat: format!("{:?}", modifier.stat).to_lowercase(),
            kind: format!("{:?}", modifier.kind).to_lowercase(),
            value: modifier.value,
        })
        .collect()
}

fn map_instance_modifiers(modifiers: &[ItemInstanceStatModifier]) -> Vec<ItemModifierView> {
    modifiers
        .iter()
        .map(|modifier| ItemModifierView {
            id: modifier.id.clone(),
            stat: format!("{:?}", modifier.stat).to_lowercase(),
            kind: format!("{:?}", modifier.kind).to_lowercase(),
            value: modifier.value,
        })
        .collect()
}

fn catalog_modifier_as_flat_line(
    modifier: &CatalogStatModifier,
) -> Option<shared::models::combat_stats::FixedStatLine> {
    match (modifier.stat, modifier.kind) {
        (
            shared::models::stat_modifier::ModifierStat::Combat(stat),
            shared::models::item_instance_attributes::StatModifierValueKind::Flat,
        ) => Some(shared::models::combat_stats::FixedStatLine {
            stat,
            value: modifier.value,
        }),
        _ => None,
    }
}

fn instance_modifier_as_flat_line(
    modifier: &ItemInstanceStatModifier,
) -> Option<shared::models::combat_stats::FixedStatLine> {
    match (modifier.stat, modifier.kind) {
        (
            shared::models::stat_modifier::ModifierStat::Combat(stat),
            shared::models::item_instance_attributes::StatModifierValueKind::Flat,
        ) => Some(shared::models::combat_stats::FixedStatLine {
            stat,
            value: modifier.value,
        }),
        _ => None,
    }
}
