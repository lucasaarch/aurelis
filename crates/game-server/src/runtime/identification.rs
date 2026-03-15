use rand::Rng;
use shared::models::{
    item_data::{EquipmentIdentificationRules, ItemData},
    item_instance_attributes::{ItemInstanceAttributes, ItemInstanceStatModifier},
};

use crate::{
    resources::internal_api::PersistedItemInstance,
    runtime::item_instance_rolls::{parse_attributes, roll_modifier_definitions},
};

pub fn identify_item_instance<R: Rng + ?Sized>(
    item_data: &'static ItemData,
    item_instance: &PersistedItemInstance,
    rng: &mut R,
) -> Result<ItemInstanceAttributes, String> {
    let rules = item_data
        .kind
        .identification()
        .ok_or_else(|| format!("item '{}' does not support identification", item_data.slug))?;

    let mut attributes = parse_attributes(&item_instance.attributes_json)?;
    if attributes.identified {
        return Err(format!(
            "item instance '{}' is already identified",
            item_instance.id
        ));
    }

    attributes.identified = true;
    attributes.roll_bias = Some(rules.bias);
    attributes.additional_effects = roll_additional_effects(rules, rng);

    Ok(attributes)
}

fn roll_additional_effects<R: Rng + ?Sized>(
    rules: EquipmentIdentificationRules,
    rng: &mut R,
) -> Vec<ItemInstanceStatModifier> {
    roll_modifier_definitions(
        rules.additional_effect_count,
        rules.additional_effect_pool,
        rules.bias,
        rng,
    )
}
