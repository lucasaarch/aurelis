use shared::{
    data::{characters::find_character_by_slug, cities::find_item_by_slug},
    models::{
        combat_stats::CombatStats, equipment_slot::EquipmentSlot, item_data::ItemKind,
    },
};

use crate::{
    resources::internal_api::PlayableCharacterSnapshot,
    runtime::character::{
        ResolvedEquippedItem, RuntimeCharacter, RuntimeLoadout, RuntimeStatBlock,
    },
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

    let mut equipped = std::collections::HashMap::new();
    let mut equipment_stats = CombatStats::ZERO;

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
            format!("equippable item '{}' is missing fixed stats", item_data.slug)
        })?;
        equipment_stats.add_lines(fixed_stats);

        equipped.insert(
            slot,
            ResolvedEquippedItem {
                item_instance_id: item_instance.id,
                item_slug: item_instance.item_slug.clone(),
                item_data,
            },
        );
    }

    let mut final_stats = base_stats;
    final_stats += class_stats;
    final_stats += equipment_stats;

    Ok(RuntimeCharacter {
        account_id: snapshot.account_id,
        character_id: snapshot.character_id,
        name: snapshot.name.clone(),
        base_character_slug: snapshot.base_character_slug.clone(),
        current_class_slug: snapshot.current_class_slug.clone(),
        level: snapshot.level,
        experience: snapshot.experience,
        credits: snapshot.credits,
        loadout: RuntimeLoadout { equipped },
        stats: RuntimeStatBlock {
            base: base_stats,
            from_class: class_stats,
            from_equipment: equipment_stats,
            final_stats,
        },
    })
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
