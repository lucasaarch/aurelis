use std::cell::RefCell;

use uuid::Uuid;

use crate::{
    resources::internal_api::{
        PersistedEquipment, PersistedInventory, PersistedInventoryItem, PersistedItemInstance,
        PersistedItemInstanceGem, PlayableCharacterSnapshot,
    },
    runtime::equipment_change::{CharacterSnapshotApi, equip_item, unequip_item},
};
use shared::models::character_data::CharacterSkillUnlocks;

#[test]
fn equips_item_and_returns_reloaded_snapshot() {
    let before = snapshot_with_inventory_weapon();
    let after = snapshot_with_equipped_weapon();
    let api = FakeSnapshotApi::new(after.clone());

    let reloaded = equip_item(&api, &before, "equipment", 0).expect("equip should succeed");

    assert_eq!(reloaded.equipment.len(), 1);
    assert_eq!(reloaded.equipment[0].slot, "weapon");
    assert!(api.equip_calls.borrow().iter().any(|call| {
        call.inventory_type == "equipment"
            && call.slot == 0
            && call.character_id == before.character_id
            && call.account_id == before.account_id
    }));
}

#[test]
fn rejects_equip_when_inventory_slot_is_empty() {
    let before = snapshot_with_inventory_weapon();
    let api = FakeSnapshotApi::new(snapshot_with_equipped_weapon());

    let err = equip_item(&api, &before, "equipment", 9).expect_err("equip should fail");

    assert!(err.contains("inventory slot is empty"));
    assert!(api.equip_calls.borrow().is_empty());
}

#[test]
fn rejects_equip_when_character_level_is_too_low() {
    let mut before = snapshot_with_inventory_weapon();
    before.level = 0;
    let api = FakeSnapshotApi::new(snapshot_with_equipped_weapon());

    let err = equip_item(&api, &before, "equipment", 0).expect_err("equip should fail");

    assert!(err.contains("below required level"));
    assert!(api.equip_calls.borrow().is_empty());
}

#[test]
fn unequips_item_and_returns_reloaded_snapshot() {
    let before = snapshot_with_equipped_weapon();
    let after = snapshot_with_inventory_weapon();
    let api = FakeSnapshotApi::new(after.clone());

    let reloaded = unequip_item(&api, &before, "weapon").expect("unequip should succeed");

    assert!(reloaded.equipment.is_empty());
    assert!(api.unequip_calls.borrow().iter().any(|call| {
        call.equipment_slot == "weapon"
            && call.character_id == before.character_id
            && call.account_id == before.account_id
    }));
}

#[test]
fn rejects_unequip_when_slot_is_empty() {
    let before = snapshot_with_inventory_weapon();
    let api = FakeSnapshotApi::new(snapshot_with_inventory_weapon());

    let err = unequip_item(&api, &before, "weapon").expect_err("unequip should fail");

    assert!(err.contains("equipment slot 'weapon' is empty"));
    assert!(api.unequip_calls.borrow().is_empty());
}

#[derive(Clone)]
struct EquipCall {
    account_id: Uuid,
    character_id: Uuid,
    inventory_type: String,
    slot: i16,
}

#[derive(Clone)]
struct UnequipCall {
    account_id: Uuid,
    character_id: Uuid,
    equipment_slot: String,
}

struct FakeSnapshotApi {
    reloaded_snapshot: PlayableCharacterSnapshot,
    equip_calls: RefCell<Vec<EquipCall>>,
    unequip_calls: RefCell<Vec<UnequipCall>>,
}

impl FakeSnapshotApi {
    fn new(reloaded_snapshot: PlayableCharacterSnapshot) -> Self {
        Self {
            reloaded_snapshot,
            equip_calls: RefCell::new(Vec::new()),
            unequip_calls: RefCell::new(Vec::new()),
        }
    }
}

impl CharacterSnapshotApi for FakeSnapshotApi {
    fn equip_inventory_item(
        &self,
        account_id: Uuid,
        character_id: Uuid,
        inventory_type: String,
        slot: i16,
    ) -> Result<(), String> {
        self.equip_calls.borrow_mut().push(EquipCall {
            account_id,
            character_id,
            inventory_type,
            slot,
        });
        Ok(())
    }

    fn unequip_item(
        &self,
        account_id: Uuid,
        character_id: Uuid,
        equipment_slot: String,
    ) -> Result<(), String> {
        self.unequip_calls.borrow_mut().push(UnequipCall {
            account_id,
            character_id,
            equipment_slot,
        });
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

fn snapshot_with_inventory_weapon() -> PlayableCharacterSnapshot {
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
            items: vec![PersistedInventoryItem {
                id: Uuid::parse_str("dddddddd-dddd-dddd-dddd-dddddddddddd").unwrap(),
                inventory_id: Uuid::parse_str("cccccccc-cccc-cccc-cccc-cccccccccccc").unwrap(),
                inventory_type: "equipment".to_string(),
                slot_index: 0,
                quantity: 1,
                item_instance_id: Some(
                    Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap(),
                ),
                item_id: None,
                item_slug: Some("kael_training_blade".to_string()),
            }],
        }],
        equipment: vec![],
        item_instances: vec![weapon_item_instance()],
    }
}

fn snapshot_with_equipped_weapon() -> PlayableCharacterSnapshot {
    PlayableCharacterSnapshot {
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
        ..snapshot_with_inventory_weapon()
    }
}

fn weapon_item_instance() -> PersistedItemInstance {
    PersistedItemInstance {
        id: Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap(),
        item_id: Uuid::parse_str("eeeeeeee-eeee-eeee-eeee-eeeeeeeeeeee").unwrap(),
        item_slug: "kael_training_blade".to_string(),
        inventory_type: "equipment".to_string(),
        refinement: 0,
        bonus_gem_slots: 0,
        attributes_json: "{}".to_string(),
        in_shared_storage: false,
        in_trade: false,
        gems: Vec::<PersistedItemInstanceGem>::new(),
    }
}
