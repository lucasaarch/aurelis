use rand::{SeedableRng, rngs::StdRng};
use uuid::Uuid;

use crate::{
    resources::internal_api::{
        PersistedEquipment, PersistedInventory, PersistedItemInstance, PlayableCharacterSnapshot,
    },
    runtime::refinement::{RefinementOutcome, roll_refinement_outcome, validate_refinement_target},
};
use shared::models::character_data::CharacterSkillUnlocks;

#[test]
fn validates_refinement_target_from_equipped_item() {
    let snapshot = equipped_snapshot(3);

    let (item_instance_id, refinement) =
        validate_refinement_target(&snapshot, "weapon").expect("target should validate");

    assert_eq!(
        item_instance_id,
        Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap()
    );
    assert_eq!(refinement, 3);
}

#[test]
fn rejects_refinement_when_slot_is_empty() {
    let snapshot = PlayableCharacterSnapshot {
        equipment: vec![],
        ..equipped_snapshot(0)
    };

    let err = validate_refinement_target(&snapshot, "weapon").expect_err("should fail");

    assert!(err.contains("equipment slot 'weapon' is empty"));
}

#[test]
fn rejects_refinement_when_item_is_already_at_cap() {
    let snapshot = equipped_snapshot(7);

    let err = validate_refinement_target(&snapshot, "weapon").expect_err("should fail");

    assert!(err.contains("already at max refinement"));
}

#[test]
fn rolls_successful_refinement_from_seeded_rng() {
    let mut rng = StdRng::seed_from_u64(7);
    let outcome = roll_refinement_outcome(0, &mut rng).expect("roll should work");

    assert!(matches!(
        outcome,
        RefinementOutcome::Success {
            old_refinement: 0,
            new_refinement: 1
        }
    ));
}

fn equipped_snapshot(refinement: i16) -> PlayableCharacterSnapshot {
    PlayableCharacterSnapshot {
        account_id: Uuid::parse_str("aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa").unwrap(),
        character_id: Uuid::parse_str("bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb").unwrap(),
        name: "Kaelzinho".to_string(),
        base_character_slug: "kael".to_string(),
        current_class_slug: "kael_royal_sentinel".to_string(),
        level: 20,
        experience: 0,
        credits: 0,
        skill_unlocks: CharacterSkillUnlocks::default(),
        inventories: vec![PersistedInventory {
            id: Uuid::parse_str("cccccccc-cccc-cccc-cccc-cccccccccccc").unwrap(),
            inventory_type: "equipment".to_string(),
            capacity: 56,
            items: vec![],
        }],
        equipment: vec![PersistedEquipment {
            slot: "weapon".to_string(),
            item_instance_id: Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap(),
        }],
        item_instances: vec![PersistedItemInstance {
            id: Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap(),
            item_id: Uuid::parse_str("eeeeeeee-eeee-eeee-eeee-eeeeeeeeeeee").unwrap(),
            item_slug: "kael_training_blade".to_string(),
            inventory_type: "equipment".to_string(),
            refinement,
            bonus_gem_slots: 0,
            attributes_json: "{}".to_string(),
            in_shared_storage: false,
            in_trade: false,
            gems: vec![],
        }],
    }
}
