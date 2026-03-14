use uuid::Uuid;

use crate::{
    resources::internal_api::{
        PersistedEquipment, PersistedInventory, PersistedInventoryItem, PersistedItemInstance,
        PersistedItemInstanceGem, PlayableCharacterSnapshot,
    },
    runtime::builder::build_runtime_character,
};

#[test]
fn builds_runtime_character_with_base_stats_only() {
    let snapshot = snapshot_base_only();

    let runtime = build_runtime_character(&snapshot).expect("runtime build should succeed");

    assert_eq!(runtime.stats.base.core.hp, 520);
    assert_eq!(runtime.stats.base.core.physical_atk, 48);
    assert_eq!(runtime.stats.from_class.core.hp, 0);
    assert_eq!(runtime.stats.from_equipment.core.hp, 0);
    assert_eq!(runtime.stats.final_stats.core.hp, 520);
    assert_eq!(runtime.stats.final_stats.core.physical_def, 32);
    assert_eq!(runtime.stats.final_stats.secondary.damage_reduction, 0);
}

#[test]
fn builds_runtime_character_with_class_bonus() {
    let mut snapshot = snapshot_base_only();
    snapshot.current_class_slug = "kael_royal_sentinel".to_string();

    let runtime = build_runtime_character(&snapshot).expect("runtime build should succeed");

    assert_eq!(runtime.stats.base.core.hp, 520);
    assert_eq!(runtime.stats.from_class.core.hp, 320);
    assert_eq!(runtime.stats.from_class.core.physical_atk, 32);
    assert_eq!(runtime.stats.final_stats.core.hp, 840);
    assert_eq!(runtime.stats.final_stats.core.physical_atk, 80);
    assert_eq!(runtime.stats.final_stats.core.atk_spd, 108);
}

#[test]
fn builds_runtime_character_with_fixed_equipment_stats() {
    let mut snapshot = snapshot_base_only();
    snapshot.current_class_slug = "kael_royal_sentinel".to_string();
    snapshot.equipment = vec![
        equipment("weapon", "11111111-1111-1111-1111-111111111111"),
        equipment("chest", "22222222-2222-2222-2222-222222222222"),
        equipment("legs", "33333333-3333-3333-3333-333333333333"),
        equipment("gloves", "44444444-4444-4444-4444-444444444444"),
        equipment("shoes", "55555555-5555-5555-5555-555555555555"),
    ];
    snapshot.item_instances = vec![
        item_instance("11111111-1111-1111-1111-111111111111", "kael_training_blade"),
        item_instance("22222222-2222-2222-2222-222222222222", "kael_squire_chestplate"),
        item_instance("33333333-3333-3333-3333-333333333333", "kael_squire_legguards"),
        item_instance("44444444-4444-4444-4444-444444444444", "kael_squire_gauntlets"),
        item_instance("55555555-5555-5555-5555-555555555555", "kael_squire_boots"),
    ];

    let runtime = build_runtime_character(&snapshot).expect("runtime build should succeed");

    assert_eq!(runtime.stats.from_equipment.core.physical_atk, 42);
    assert_eq!(runtime.stats.from_equipment.core.hp, 120);
    assert_eq!(runtime.stats.from_equipment.core.physical_def, 56);
    assert_eq!(runtime.stats.from_equipment.core.magical_def, 29);
    assert_eq!(runtime.stats.from_equipment.core.atk_spd, 7);
    assert_eq!(runtime.stats.from_equipment.core.move_spd, 9);
    assert_eq!(runtime.stats.from_equipment.secondary.damage_reduction, 6);

    assert_eq!(runtime.stats.final_stats.core.hp, 960);
    assert_eq!(runtime.stats.final_stats.core.physical_atk, 122);
    assert_eq!(runtime.stats.final_stats.core.physical_def, 110);
    assert_eq!(runtime.stats.final_stats.core.magical_def, 51);
    assert_eq!(runtime.stats.final_stats.core.atk_spd, 115);
    assert_eq!(runtime.stats.final_stats.core.move_spd, 114);
    assert_eq!(runtime.stats.final_stats.secondary.damage_reduction, 6);
}

fn snapshot_base_only() -> PlayableCharacterSnapshot {
    PlayableCharacterSnapshot {
        account_id: Uuid::parse_str("aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa").unwrap(),
        character_id: Uuid::parse_str("bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb").unwrap(),
        name: "Kaelzinho".to_string(),
        base_character_slug: "kael".to_string(),
        current_class_slug: "kael".to_string(),
        level: 12,
        experience: 8450,
        credits: 3200,
        inventories: vec![PersistedInventory {
            id: Uuid::parse_str("cccccccc-cccc-cccc-cccc-cccccccccccc").unwrap(),
            inventory_type: "equipment".to_string(),
            capacity: 56,
            items: vec![PersistedInventoryItem {
                id: Uuid::parse_str("dddddddd-dddd-dddd-dddd-dddddddddddd").unwrap(),
                inventory_id: Uuid::parse_str("cccccccc-cccc-cccc-cccc-cccccccccccc").unwrap(),
                inventory_type: "equipment".to_string(),
                slot_index: 0,
                quantity: 1,
                item_instance_id: None,
                item_id: None,
                item_slug: None,
            }],
        }],
        equipment: vec![],
        item_instances: vec![],
    }
}

fn equipment(slot: &str, item_instance_id: &str) -> PersistedEquipment {
    PersistedEquipment {
        slot: slot.to_string(),
        item_instance_id: Uuid::parse_str(item_instance_id).unwrap(),
    }
}

fn item_instance(id: &str, item_slug: &str) -> PersistedItemInstance {
    PersistedItemInstance {
        id: Uuid::parse_str(id).unwrap(),
        item_id: Uuid::parse_str("eeeeeeee-eeee-eeee-eeee-eeeeeeeeeeee").unwrap(),
        item_slug: item_slug.to_string(),
        inventory_type: "equipment".to_string(),
        refinement: 0,
        gem_slots: 4,
        attributes_json: "{}".to_string(),
        in_shared_storage: false,
        in_trade: false,
        gems: Vec::<PersistedItemInstanceGem>::new(),
    }
}
