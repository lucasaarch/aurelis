use uuid::Uuid;

use crate::{
    resources::internal_api::{
        PersistedEquipment, PersistedInventory, PersistedInventoryItem, PersistedItemInstance,
        PersistedItemInstanceGem, PlayableCharacterSnapshot,
    },
    runtime::gem_socket::{GemSocketApi, socket_gem},
};
use shared::models::character_data::CharacterSkillUnlocks;
use std::cell::RefCell;

#[test]
fn overwrites_existing_gem_in_occupied_socket() {
    let api = FakeGemSocketApi::new(snapshot_after_socket());
    let snapshot = snapshot_with_gem_inventory_and_equipment(true);

    let reloaded = socket_gem(&api, &snapshot, "weapon", "material", 0, 0)
        .expect("socket overwrite should succeed");

    let weapon = reloaded
        .item_instances
        .iter()
        .find(|item| item.item_slug == "kael_training_blade")
        .expect("weapon instance");
    assert_eq!(weapon.gems.len(), 1);
    assert_eq!(weapon.gems[0].slot_index, 0);
}

#[test]
fn sockets_gem_and_returns_reloaded_snapshot() {
    let api = FakeGemSocketApi::new(snapshot_after_socket());
    let snapshot = snapshot_with_gem_inventory_and_equipment(false);

    let reloaded =
        socket_gem(&api, &snapshot, "weapon", "material", 0, 0).expect("socket should succeed");

    let weapon = reloaded
        .item_instances
        .iter()
        .find(|item| item.item_slug == "kael_training_blade")
        .expect("weapon instance");
    assert_eq!(weapon.gems.len(), 1);
    assert_eq!(weapon.gems[0].slot_index, 0);
}

struct FakeGemSocketApi {
    reloaded_snapshot: PlayableCharacterSnapshot,
    calls: RefCell<Vec<(String, String, i16, i16)>>,
}

impl FakeGemSocketApi {
    fn new(reloaded_snapshot: PlayableCharacterSnapshot) -> Self {
        Self {
            reloaded_snapshot,
            calls: RefCell::new(Vec::new()),
        }
    }
}

impl GemSocketApi for FakeGemSocketApi {
    fn socket_gem(
        &self,
        _account_id: Uuid,
        _character_id: Uuid,
        equipment_slot: String,
        inventory_type: String,
        slot: i16,
        socket_index: i16,
    ) -> Result<(), String> {
        self.calls
            .borrow_mut()
            .push((equipment_slot, inventory_type, slot, socket_index));
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

fn snapshot_with_gem_inventory_and_equipment(occupied: bool) -> PlayableCharacterSnapshot {
    let mut weapon = PersistedItemInstance {
        id: Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap(),
        item_id: Uuid::parse_str("eeeeeeee-eeee-eeee-eeee-eeeeeeeeeeee").unwrap(),
        item_slug: "kael_training_blade".to_string(),
        inventory_type: "equipment".to_string(),
        refinement: 0,
        bonus_gem_slots: 0,
        attributes_json: "{}".to_string(),
        in_shared_storage: false,
        in_trade: false,
        gems: vec![],
    };
    if occupied {
        weapon.gems.push(PersistedItemInstanceGem {
            slot_index: 0,
            gem_instance_id: Uuid::parse_str("22222222-2222-2222-2222-222222222222").unwrap(),
        });
    }

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
            inventory_type: "material".to_string(),
            capacity: 56,
            items: vec![PersistedInventoryItem {
                id: Uuid::parse_str("dddddddd-dddd-dddd-dddd-dddddddddddd").unwrap(),
                inventory_id: Uuid::parse_str("cccccccc-cccc-cccc-cccc-cccccccccccc").unwrap(),
                inventory_type: "material".to_string(),
                slot_index: 0,
                quantity: 1,
                item_instance_id: Some(
                    Uuid::parse_str("22222222-2222-2222-2222-222222222222").unwrap(),
                ),
                item_id: None,
                item_slug: Some("chaos_gem".to_string()),
            }],
        }],
        equipment: vec![PersistedEquipment {
            slot: "weapon".to_string(),
            item_instance_id: weapon.id,
        }],
        item_instances: vec![
            weapon,
            PersistedItemInstance {
                id: Uuid::parse_str("22222222-2222-2222-2222-222222222222").unwrap(),
                item_id: Uuid::parse_str("ffffffff-ffff-ffff-ffff-ffffffffffff").unwrap(),
                item_slug: "chaos_gem".to_string(),
                inventory_type: "material".to_string(),
                refinement: 0,
                bonus_gem_slots: 0,
                attributes_json: "{}".to_string(),
                in_shared_storage: false,
                in_trade: false,
                gems: vec![],
            },
        ],
    }
}

fn snapshot_after_socket() -> PlayableCharacterSnapshot {
    let mut snapshot = snapshot_with_gem_inventory_and_equipment(false);
    snapshot.inventories[0].items.clear();
    snapshot.item_instances[0]
        .gems
        .push(PersistedItemInstanceGem {
            slot_index: 0,
            gem_instance_id: Uuid::parse_str("22222222-2222-2222-2222-222222222222").unwrap(),
        });
    snapshot
}
