use rand::{SeedableRng, rngs::StdRng};
use shared::{
    data::cities::find_item_by_slug,
    models::{
        combat_stats::StatKey,
        item_instance_attributes::{EquipmentRollBias, ItemInstanceAttributes},
        stat_modifier::ModifierStat,
    },
};
use uuid::Uuid;

use crate::{
    resources::internal_api::PersistedItemInstance, runtime::identification::identify_item_instance,
};

#[test]
fn identifies_item_instance_using_catalog_rules() {
    let item_data = find_item_by_slug("kael_training_blade").expect("catalog item should exist");
    let item_instance = base_weapon_instance("{}");
    let mut rng = StdRng::seed_from_u64(42);

    let attributes =
        identify_item_instance(item_data, &item_instance, &mut rng).expect("identify should work");

    assert!(attributes.identified);
    assert_eq!(attributes.roll_bias, Some(EquipmentRollBias::Physical));
    assert_eq!(attributes.additional_effects.len(), 3);
    assert!(
        attributes
            .additional_effects
            .iter()
            .any(|effect| effect.stat == ModifierStat::Combat(StatKey::PhysicalAttackLevel))
    );
}

#[test]
fn refuses_to_identify_an_already_identified_item() {
    let item_data = find_item_by_slug("kael_training_blade").expect("catalog item should exist");
    let existing = serde_json::to_string(&ItemInstanceAttributes {
        identified: true,
        roll_bias: Some(EquipmentRollBias::Physical),
        reroll_count: 1,
        additional_effects: vec![],
    })
    .unwrap();
    let item_instance = base_weapon_instance(&existing);
    let mut rng = StdRng::seed_from_u64(7);

    let error = identify_item_instance(item_data, &item_instance, &mut rng)
        .expect_err("identify should fail for identified item");

    assert!(error.contains("already identified"));
}

fn base_weapon_instance(attributes_json: &str) -> PersistedItemInstance {
    PersistedItemInstance {
        id: Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap(),
        item_id: Uuid::parse_str("eeeeeeee-eeee-eeee-eeee-eeeeeeeeeeee").unwrap(),
        item_slug: "kael_training_blade".to_string(),
        inventory_type: "equipment".to_string(),
        refinement: 0,
        bonus_gem_slots: 0,
        attributes_json: attributes_json.to_string(),
        in_shared_storage: false,
        in_trade: false,
        gems: vec![],
    }
}
