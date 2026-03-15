use uuid::Uuid;

use crate::{
    resources::internal_api::{
        PersistedEquipment, PersistedInventory, PersistedInventoryItem, PersistedItemInstance,
        PersistedItemInstanceGem, PlayableCharacterSnapshot,
    },
    runtime::{builder::build_runtime_character, client_snapshot::build_character_snapshot_view},
};
use shared::{
    models::character_data::CharacterSkillUnlocks,
    net::{ClientMessage, ServerMessage},
};

#[test]
fn serializes_and_deserializes_character_snapshot_message() {
    let (runtime, snapshot) = full_snapshot_fixture();
    let snapshot_view =
        build_character_snapshot_view(&runtime, &snapshot).expect("snapshot view should build");

    let encoded = bincode::serialize(&ServerMessage::CharacterSnapshotLoaded {
        snapshot: snapshot_view.clone(),
    })
    .expect("server message should serialize");
    let decoded: ServerMessage =
        bincode::deserialize(&encoded).expect("server message should deserialize");

    match decoded {
        ServerMessage::CharacterSnapshotLoaded { snapshot } => {
            assert_eq!(snapshot.character_id, snapshot_view.character_id);
            assert_eq!(
                snapshot.stats.final_combat_stats,
                snapshot_view.stats.final_combat_stats
            );
            assert_eq!(snapshot.inventories.len(), snapshot_view.inventories.len());
        }
        other => panic!("unexpected decoded message: {other:?}"),
    }

    let client_encoded = bincode::serialize(&ClientMessage::UseSkill {
        skill_slug: "sentinel_steel_pulse".to_string(),
    })
    .expect("client message should serialize");
    let _: ClientMessage =
        bincode::deserialize(&client_encoded).expect("client message should deserialize");
}

#[test]
fn builds_client_snapshot_with_resolved_inventory_and_equipment() {
    let (runtime, snapshot) = full_snapshot_fixture();

    let client_snapshot =
        build_character_snapshot_view(&runtime, &snapshot).expect("snapshot view should build");

    assert_eq!(client_snapshot.character_id, runtime.character_id);
    assert_eq!(client_snapshot.affinity, runtime.combat_affinity);
    assert_eq!(
        client_snapshot.stats.final_combat_stats,
        runtime.stats.final_stats
    );
    assert_eq!(
        client_snapshot.stats.current_hp,
        runtime.resources.current_hp
    );
    assert_eq!(
        client_snapshot.stats.current_mp,
        runtime.resources.current_mp
    );
    assert!(
        client_snapshot
            .available_skills
            .iter()
            .any(|skill| skill.slug == "sentinel_discipline")
    );
    assert_eq!(client_snapshot.inventories.len(), 2);
    assert_eq!(client_snapshot.equipped.len(), 5);

    let equipment_inventory = client_snapshot
        .inventories
        .iter()
        .find(|inventory| inventory.inventory_type == "equipment")
        .expect("equipment inventory should be present");
    let weapon_slot = equipment_inventory
        .slots
        .iter()
        .find(|slot| slot.slot_index == 0)
        .expect("weapon inventory slot should exist");
    let weapon = weapon_slot
        .item
        .as_ref()
        .expect("weapon slot should resolve to item view");

    assert_eq!(weapon.item_slug, "kael_training_blade");
    assert_eq!(weapon.refinement, 3);
    assert_eq!(weapon.socketed_gems.len(), 1);
    assert_eq!(weapon.additional_effects.len(), 3);
    assert_eq!(weapon.resolved_combat_stats.core.physical_atk, 54);
    assert_eq!(
        weapon.resolved_combat_stats.secondary.physical_attack_level,
        375
    );

    let socketed_gem = &weapon.socketed_gems[0];
    assert_eq!(socketed_gem.item_slug, "chaos_gem");
    assert_eq!(socketed_gem.additional_effects.len(), 1);
    assert_eq!(socketed_gem.resolved_combat_stats.core.hp, 72);
}

fn full_snapshot_fixture() -> (
    crate::runtime::character::RuntimeCharacter,
    PlayableCharacterSnapshot,
) {
    let snapshot = PlayableCharacterSnapshot {
        account_id: Uuid::parse_str("aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa").unwrap(),
        character_id: Uuid::parse_str("bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb").unwrap(),
        name: "Kaelzinho".to_string(),
        base_character_slug: "kael".to_string(),
        current_class_slug: "kael_royal_sentinel".to_string(),
        level: 20,
        experience: 8450,
        credits: 3200,
        skill_unlocks: CharacterSkillUnlocks::default(),
        inventories: vec![
            PersistedInventory {
                id: Uuid::parse_str("cccccccc-cccc-cccc-cccc-cccccccccccc").unwrap(),
                inventory_type: "equipment".to_string(),
                capacity: 56,
                items: vec![
                    PersistedInventoryItem {
                        id: Uuid::parse_str("dddddddd-dddd-dddd-dddd-dddddddddddd").unwrap(),
                        inventory_id: Uuid::parse_str("cccccccc-cccc-cccc-cccc-cccccccccccc")
                            .unwrap(),
                        inventory_type: "equipment".to_string(),
                        slot_index: 0,
                        quantity: 1,
                        item_instance_id: Some(
                            Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap(),
                        ),
                        item_id: None,
                        item_slug: Some("kael_training_blade".to_string()),
                    },
                    PersistedInventoryItem {
                        id: Uuid::parse_str("dddddddd-dddd-dddd-dddd-000000000001").unwrap(),
                        inventory_id: Uuid::parse_str("cccccccc-cccc-cccc-cccc-cccccccccccc")
                            .unwrap(),
                        inventory_type: "equipment".to_string(),
                        slot_index: 1,
                        quantity: 1,
                        item_instance_id: Some(
                            Uuid::parse_str("22222222-2222-2222-2222-222222222222").unwrap(),
                        ),
                        item_id: None,
                        item_slug: Some("kael_squire_chestplate".to_string()),
                    },
                ],
            },
            PersistedInventory {
                id: Uuid::parse_str("cccccccc-cccc-cccc-cccc-000000000002").unwrap(),
                inventory_type: "material".to_string(),
                capacity: 56,
                items: vec![PersistedInventoryItem {
                    id: Uuid::parse_str("dddddddd-dddd-dddd-dddd-000000000003").unwrap(),
                    inventory_id: Uuid::parse_str("cccccccc-cccc-cccc-cccc-000000000002").unwrap(),
                    inventory_type: "material".to_string(),
                    slot_index: 0,
                    quantity: 1,
                    item_instance_id: Some(
                        Uuid::parse_str("88888888-8888-8888-8888-888888888888").unwrap(),
                    ),
                    item_id: None,
                    item_slug: Some("chaos_gem".to_string()),
                }],
            },
        ],
        equipment: vec![
            equipment("weapon", "11111111-1111-1111-1111-111111111111"),
            equipment("chest", "22222222-2222-2222-2222-222222222222"),
            equipment("legs", "33333333-3333-3333-3333-333333333333"),
            equipment("gloves", "44444444-4444-4444-4444-444444444444"),
            equipment("shoes", "55555555-5555-5555-5555-555555555555"),
        ],
        item_instances: vec![
            weapon_instance(),
            item_instance(
                "22222222-2222-2222-2222-222222222222",
                "kael_squire_chestplate",
            ),
            item_instance(
                "33333333-3333-3333-3333-333333333333",
                "kael_squire_legguards",
            ),
            item_instance(
                "44444444-4444-4444-4444-444444444444",
                "kael_squire_gauntlets",
            ),
            item_instance("55555555-5555-5555-5555-555555555555", "kael_squire_boots"),
            rolled_gem_instance(),
        ],
    };

    let runtime = build_runtime_character(&snapshot).expect("runtime should build");
    (runtime, snapshot)
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
        bonus_gem_slots: 0,
        attributes_json: "{}".to_string(),
        in_shared_storage: false,
        in_trade: false,
        gems: vec![],
    }
}

fn weapon_instance() -> PersistedItemInstance {
    let mut item = item_instance(
        "11111111-1111-1111-1111-111111111111",
        "kael_training_blade",
    );
    item.refinement = 3;
    item.attributes_json = r#"{
        "identified": true,
        "roll_bias": "physical",
        "reroll_count": 1,
        "additional_effects": [
            { "id": "roll_phys_level", "stat": "physical_attack_level", "kind": "flat", "value": 375 },
            { "id": "roll_crit_damage", "stat": "crit_damage", "kind": "flat", "value": 5 },
            { "id": "roll_phys_atk_pct", "stat": "physical_atk", "kind": "percent", "value": 1000 }
        ]
    }"#
    .to_string();
    item.gems = vec![PersistedItemInstanceGem {
        slot_index: 0,
        gem_instance_id: Uuid::parse_str("88888888-8888-8888-8888-888888888888").unwrap(),
    }];
    item
}

fn rolled_gem_instance() -> PersistedItemInstance {
    let mut item = item_instance("88888888-8888-8888-8888-888888888888", "chaos_gem");
    item.inventory_type = "material".to_string();
    item.attributes_json = r#"{
        "identified": true,
        "roll_bias": "neutral",
        "reroll_count": 0,
        "additional_effects": [
            { "id": "gem_roll_hp", "stat": "hp", "kind": "flat", "value": 72 }
        ]
    }"#
    .to_string();
    item
}
