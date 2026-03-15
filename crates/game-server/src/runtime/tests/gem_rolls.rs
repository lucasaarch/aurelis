use rand::{SeedableRng, rngs::StdRng};
use shared::{
    data::cities::find_item_by_slug,
    models::{item_instance_attributes::EquipmentRollBias, stat_modifier::ModifierStat},
};
use uuid::Uuid;

use crate::{
    resources::internal_api::PersistedItemInstance, runtime::gem_rolls::roll_gem_instance_effects,
};

#[test]
fn rolls_gem_instance_effects_from_catalog_pool() {
    let gem_data = find_item_by_slug("chaos_gem").expect("catalog gem should exist");
    let gem_instance = base_gem_instance("{}");
    let mut rng = StdRng::seed_from_u64(42);

    let attributes =
        roll_gem_instance_effects(gem_data, &gem_instance, &mut rng).expect("gem roll should work");

    assert!(attributes.identified);
    assert_eq!(attributes.roll_bias, Some(EquipmentRollBias::Neutral));
    assert_eq!(attributes.additional_effects.len(), 1);
    assert!(matches!(
        attributes.additional_effects[0].stat,
        ModifierStat::Combat(_)
    ));
}

#[test]
fn refuses_to_roll_a_gem_twice() {
    let gem_data = find_item_by_slug("chaos_gem").expect("catalog gem should exist");
    let gem_instance = base_gem_instance(
        r#"{"identified":true,"roll_bias":"neutral","reroll_count":0,"additional_effects":[]}"#,
    );
    let mut rng = StdRng::seed_from_u64(7);

    let error = roll_gem_instance_effects(gem_data, &gem_instance, &mut rng)
        .expect_err("rolling an already-rolled gem should fail");

    assert!(error.contains("already rolled"));
}

fn base_gem_instance(attributes_json: &str) -> PersistedItemInstance {
    PersistedItemInstance {
        id: Uuid::parse_str("88888888-8888-8888-8888-888888888888").unwrap(),
        item_id: Uuid::parse_str("99999999-9999-9999-9999-999999999999").unwrap(),
        item_slug: "chaos_gem".to_string(),
        inventory_type: "material".to_string(),
        refinement: 0,
        bonus_gem_slots: 0,
        attributes_json: attributes_json.to_string(),
        in_shared_storage: false,
        in_trade: false,
        gems: vec![],
    }
}
