use std::cell::RefCell;

use rand::{SeedableRng, rngs::StdRng};
use uuid::Uuid;

use crate::{
    resources::internal_api::{
        PersistedEquipment, PersistedInventory, PersistedInventoryItem, PersistedItemInstance,
        PlayableCharacterSnapshot,
    },
    runtime::use_item::{UseItemApi, UseItemRequest, use_item_with_rng},
};
use shared::{
    data::cities::find_item_by_slug,
    models::{
        character_data::CharacterSkillUnlocks, item_instance_attributes::ItemInstanceAttributes,
        skill_data::CharacterSkillUnlockTier,
    },
};

#[test]
fn rerolls_identified_equipment_and_consumes_scroll() {
    let snapshot = reroll_snapshot(true);
    let api = FakeUseItemApi::new(snapshot.clone());
    let mut rng = StdRng::seed_from_u64(7);

    let _ = use_item_with_rng(
        UseItemRequest {
            api: &api,
            snapshot: &snapshot,
            inventory_type: "special",
            slot: 0,
            target_equipment_slot: Some("weapon"),
        },
        &mut rng,
    )
    .expect("reroll should succeed");

    let persisted = api
        .persisted
        .borrow()
        .clone()
        .expect("persisted item state should be recorded");
    assert_eq!(
        persisted.0,
        Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap()
    );
    let attrs: ItemInstanceAttributes =
        serde_json::from_str(&persisted.2).expect("persisted attrs should deserialize");
    assert!(attrs.identified);
    assert_eq!(attrs.reroll_count, 1);
    assert_eq!(attrs.additional_effects.len(), 3);

    let consumed = api
        .consumed
        .borrow()
        .clone()
        .expect("scroll should be consumed");
    assert_eq!(consumed.0, "special");
    assert_eq!(consumed.1, 0);
    assert_eq!(consumed.2, 1);
}

#[test]
fn rejects_reroll_for_unidentified_equipment() {
    let snapshot = reroll_snapshot(false);
    let api = FakeUseItemApi::new(snapshot.clone());
    let mut rng = StdRng::seed_from_u64(7);

    let err = use_item_with_rng(
        UseItemRequest {
            api: &api,
            snapshot: &snapshot,
            inventory_type: "special",
            slot: 0,
            target_equipment_slot: Some("weapon"),
        },
        &mut rng,
    )
    .err()
    .expect("reroll should fail");

    assert!(err.contains("identified"));
    assert!(api.persisted.borrow().is_none());
    assert!(api.consumed.borrow().is_none());
}

#[test]
fn rejects_reroll_when_target_slot_is_missing() {
    let snapshot = reroll_snapshot(true);
    let api = FakeUseItemApi::new(snapshot.clone());
    let mut rng = StdRng::seed_from_u64(7);

    let err = use_item_with_rng(
        UseItemRequest {
            api: &api,
            snapshot: &snapshot,
            inventory_type: "special",
            slot: 0,
            target_equipment_slot: None,
        },
        &mut rng,
    )
    .err()
    .expect("reroll should fail");

    assert!(err.contains("target equipment slot"));
}

struct FakeUseItemApi {
    reloaded_snapshot: PlayableCharacterSnapshot,
    persisted: RefCell<Option<(Uuid, i16, String)>>,
    consumed: RefCell<Option<(String, i16, i16)>>,
}

impl FakeUseItemApi {
    fn new(reloaded_snapshot: PlayableCharacterSnapshot) -> Self {
        Self {
            reloaded_snapshot,
            persisted: RefCell::new(None),
            consumed: RefCell::new(None),
        }
    }
}

impl UseItemApi for FakeUseItemApi {
    fn unlock_character_skill_tier(
        &self,
        _account_id: Uuid,
        _character_id: Uuid,
        _tier: CharacterSkillUnlockTier,
    ) -> Result<(), String> {
        Ok(())
    }

    fn persist_item_instance_state(
        &self,
        _account_id: Uuid,
        _character_id: Uuid,
        item_instance_id: Uuid,
        bonus_gem_slots: i16,
        attributes_json: String,
    ) -> Result<(), String> {
        *self.persisted.borrow_mut() = Some((item_instance_id, bonus_gem_slots, attributes_json));
        Ok(())
    }

    fn consume_inventory_item(
        &self,
        _account_id: Uuid,
        _character_id: Uuid,
        inventory_type: String,
        slot: i16,
        quantity: i16,
    ) -> Result<(), String> {
        *self.consumed.borrow_mut() = Some((inventory_type, slot, quantity));
        Ok(())
    }

    fn load_playable_character(
        &self,
        _account_id: Uuid,
        _character_id: Uuid,
    ) -> Result<PlayableCharacterSnapshot, String> {
        Ok(self.reloaded_snapshot.clone())
    }
}

fn reroll_snapshot(identified: bool) -> PlayableCharacterSnapshot {
    let blade = find_item_by_slug("kael_training_blade").expect("blade item should exist");
    assert!(blade.kind.identification().is_some());

    let attributes_json = if identified {
        "{\"identified\":true,\"roll_bias\":\"physical\",\"reroll_count\":0,\"additional_effects\":[{\"id\":\"old_crit_damage\",\"stat\":\"crit_damage\",\"kind\":\"flat\",\"value\":5}]}"
            .to_string()
    } else {
        "{}".to_string()
    };

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
            inventory_type: "special".to_string(),
            capacity: 56,
            items: vec![PersistedInventoryItem {
                id: Uuid::parse_str("dddddddd-dddd-dddd-dddd-dddddddddddd").unwrap(),
                inventory_id: Uuid::parse_str("cccccccc-cccc-cccc-cccc-cccccccccccc").unwrap(),
                inventory_type: "special".to_string(),
                slot_index: 0,
                quantity: 1,
                item_instance_id: None,
                item_id: None,
                item_slug: Some("equipment_reroll_scroll".to_string()),
            }],
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
            refinement: 0,
            bonus_gem_slots: 0,
            attributes_json,
            in_shared_storage: false,
            in_trade: false,
            gems: vec![],
        }],
    }
}
