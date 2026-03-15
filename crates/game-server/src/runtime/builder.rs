use shared::{
    data::{
        characters::{all_skills_for_character, find_character_by_slug},
        cities::find_item_by_slug,
    },
    models::{
        combat_stats::CombatStats,
        equipment_slot::EquipmentSlot,
        item_data::{CatalogStatModifier, ItemKind},
        item_instance_attributes::{
            ItemInstanceAttributes, ItemInstanceStatModifier, StatModifierValueKind,
        },
        reward_stats::RewardStats,
        stat_modifier::ModifierStat,
    },
};

use crate::{
    resources::internal_api::PlayableCharacterSnapshot,
    runtime::character::{
        ResolvedEquippedItem, RuntimeCharacter, RuntimeLoadout, RuntimeRewardBlock,
        RuntimeStatBlock,
    },
    runtime::modifier::{ModifierDuration, ModifierSource, RuntimeModifier, StatModifierOp},
    runtime::skill_effects::build_passive_skill_modifiers,
};

pub fn build_runtime_character(
    snapshot: &PlayableCharacterSnapshot,
) -> Result<RuntimeCharacter, String> {
    let character = find_character_by_slug(&snapshot.base_character_slug).ok_or_else(|| {
        format!(
            "unknown base character slug '{}'",
            snapshot.base_character_slug
        )
    })?;

    let class_stats = if snapshot.current_class_slug == snapshot.base_character_slug {
        CombatStats::ZERO
    } else {
        let class = character
            .find_class_by_slug(&snapshot.current_class_slug)
            .ok_or_else(|| {
                format!(
                    "unknown class slug '{}' for character '{}'",
                    snapshot.current_class_slug, snapshot.base_character_slug
                )
            })?;
        class.stat_bonuses.into()
    };

    let base_stats: CombatStats = character.base_stats.into();
    let combat_affinity = character
        .affinity_for_current_class(&snapshot.current_class_slug)
        .ok_or_else(|| {
            format!(
                "unable to resolve affinity for current class '{}'",
                snapshot.current_class_slug
            )
        })?;
    let available_skills = all_skills_for_character(&snapshot.base_character_slug)
        .into_iter()
        .filter(|skill| {
            skill.is_unlocked_for(
                character,
                &snapshot.current_class_slug,
                snapshot.level,
                &snapshot.skill_unlocks,
            )
        })
        .collect::<Vec<_>>();
    let available_skill_slugs = available_skills
        .iter()
        .map(|skill| skill.slug.to_string())
        .collect::<Vec<_>>();

    let mut equipped = std::collections::HashMap::new();
    let mut equipment_stats = CombatStats::ZERO;
    let mut persistent_modifiers = build_passive_skill_modifiers(&available_skills);

    for equipped_item in &snapshot.equipment {
        let item_instance = snapshot
            .item_instances
            .iter()
            .find(|item| item.id == equipped_item.item_instance_id)
            .ok_or_else(|| {
                format!(
                    "missing item instance '{}' for equipped slot '{}'",
                    equipped_item.item_instance_id, equipped_item.slot
                )
            })?;

        let item_data = find_item_by_slug(&item_instance.item_slug).ok_or_else(|| {
            format!(
                "unknown item slug '{}' for item instance '{}'",
                item_instance.item_slug, item_instance.id
            )
        })?;

        let slot = parse_equipment_slot(&equipped_item.slot)?;
        let item_slot = item_data.kind.equipment_slot().ok_or_else(|| {
            format!(
                "item '{}' is not equippable but is equipped in slot '{}'",
                item_data.slug, equipped_item.slot
            )
        })?;

        if item_slot != slot {
            return Err(format!(
                "item '{}' expects slot '{:?}' but is equipped in '{}'",
                item_data.slug, item_slot, equipped_item.slot
            ));
        }

        let fixed_stats = item_data.kind.fixed_stats().ok_or_else(|| {
            format!(
                "equippable item '{}' is missing fixed stats",
                item_data.slug
            )
        })?;
        equipment_stats.add_lines(fixed_stats);
        persistent_modifiers.extend(build_fixed_effect_modifiers(
            item_instance.id,
            item_data.kind.fixed_special_effects().unwrap_or_default(),
        ));
        persistent_modifiers.extend(build_instance_attribute_modifiers(item_instance)?);

        equipped.insert(
            slot,
            ResolvedEquippedItem {
                item_instance_id: item_instance.id,
                item_slug: item_instance.item_slug.clone(),
                item_data,
            },
        );
    }

    let mut runtime_character = RuntimeCharacter {
        account_id: snapshot.account_id,
        character_id: snapshot.character_id,
        name: snapshot.name.clone(),
        base_character_slug: snapshot.base_character_slug.clone(),
        current_class_slug: snapshot.current_class_slug.clone(),
        combat_affinity,
        level: snapshot.level,
        experience: snapshot.experience,
        credits: snapshot.credits,
        skill_unlocks: snapshot.skill_unlocks,
        available_skill_slugs,
        loadout: RuntimeLoadout { equipped },
        persistent_modifiers,
        timed_modifiers: vec![],
        stats: RuntimeStatBlock {
            base: base_stats,
            from_class: class_stats,
            from_equipment: equipment_stats,
            from_persistent_modifiers: CombatStats::ZERO,
            from_timed_modifiers: CombatStats::ZERO,
            final_stats: CombatStats::ZERO,
        },
        rewards: RuntimeRewardBlock {
            base: RewardStats::ZERO,
            from_class: RewardStats::ZERO,
            from_equipment: RewardStats::ZERO,
            from_persistent_modifiers: RewardStats::ZERO,
            from_timed_modifiers: RewardStats::ZERO,
            final_stats: RewardStats::ZERO,
        },
    };
    runtime_character.recalculate_stats();

    Ok(runtime_character)
}

fn parse_equipment_slot(slot: &str) -> Result<EquipmentSlot, String> {
    match slot {
        "weapon" => Ok(EquipmentSlot::Weapon),
        "head" => Ok(EquipmentSlot::Head),
        "chest" => Ok(EquipmentSlot::Chest),
        "legs" => Ok(EquipmentSlot::Legs),
        "gloves" => Ok(EquipmentSlot::Gloves),
        "shoes" => Ok(EquipmentSlot::Shoes),
        "acc_ring_1" => Ok(EquipmentSlot::AccRing1),
        "acc_ring_2" => Ok(EquipmentSlot::AccRing2),
        "acc_necklace" => Ok(EquipmentSlot::AccNecklace),
        "acc_earrings" => Ok(EquipmentSlot::AccEarrings),
        "acc_arm" => Ok(EquipmentSlot::AccArm),
        "acc_face_bottom" => Ok(EquipmentSlot::AccFaceBottom),
        "acc_face_middle" => Ok(EquipmentSlot::AccFaceMiddle),
        "acc_face_top" => Ok(EquipmentSlot::AccFaceTop),
        "acc_bottom_piece" => Ok(EquipmentSlot::AccBottomPiece),
        "acc_top_piece" => Ok(EquipmentSlot::AccTopPiece),
        "acc_weapon" => Ok(EquipmentSlot::AccWeapon),
        "acc_support_unit" => Ok(EquipmentSlot::AccSupportUnit),
        _ => Err(format!("unknown equipment slot '{}'", slot)),
    }
}

#[allow(dead_code)]
fn _assert_item_kind_usage(item_kind: &ItemKind) -> bool {
    matches!(item_kind, ItemKind::Weapon(_) | ItemKind::Armor(_))
}

fn build_fixed_effect_modifiers(
    item_instance_id: uuid::Uuid,
    effects: &[CatalogStatModifier],
) -> Vec<RuntimeModifier> {
    effects
        .iter()
        .map(|effect| RuntimeModifier {
            source: ModifierSource::Equipment { item_instance_id },
            duration: ModifierDuration::Permanent,
            operations: vec![catalog_modifier_to_runtime_op(effect)],
        })
        .collect()
}

fn build_instance_attribute_modifiers(
    item_instance: &crate::resources::internal_api::PersistedItemInstance,
) -> Result<Vec<RuntimeModifier>, String> {
    let attributes = parse_item_instance_attributes(&item_instance.attributes_json)?;
    if !attributes.identified {
        return Ok(vec![]);
    }

    Ok(attributes
        .additional_effects
        .iter()
        .map(|effect| RuntimeModifier {
            source: ModifierSource::Equipment {
                item_instance_id: item_instance.id,
            },
            duration: ModifierDuration::Permanent,
            operations: vec![instance_modifier_to_runtime_op(effect)],
        })
        .collect())
}

fn parse_item_instance_attributes(json: &str) -> Result<ItemInstanceAttributes, String> {
    if json.trim().is_empty() || json.trim() == "{}" {
        return Ok(ItemInstanceAttributes::default());
    }

    serde_json::from_str(json)
        .map_err(|err| format!("invalid item instance attributes json: {err}"))
}

fn catalog_modifier_to_runtime_op(effect: &CatalogStatModifier) -> StatModifierOp {
    match effect.kind {
        StatModifierValueKind::Flat => StatModifierOp::AddFlat {
            stat: effect.stat,
            value: effect.value,
        },
        StatModifierValueKind::Percent => StatModifierOp::AddPercent {
            stat: effect.stat,
            value_bp: effect.value,
        },
    }
}

fn instance_modifier_to_runtime_op(effect: &ItemInstanceStatModifier) -> StatModifierOp {
    match effect.kind {
        StatModifierValueKind::Flat => StatModifierOp::AddFlat {
            stat: effect.stat,
            value: effect.value,
        },
        StatModifierValueKind::Percent => StatModifierOp::AddPercent {
            stat: effect.stat,
            value_bp: effect.value,
        },
    }
}

#[allow(dead_code)]
fn _assert_modifier_stat_usage(stat: ModifierStat) -> bool {
    matches!(stat, ModifierStat::Combat(_) | ModifierStat::Reward(_))
}
