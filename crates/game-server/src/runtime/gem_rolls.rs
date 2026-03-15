use rand::Rng;
use shared::models::{
    item_data::ItemData,
    item_instance_attributes::{EquipmentRollBias, ItemInstanceAttributes},
};

use crate::{
    resources::internal_api::PersistedItemInstance,
    runtime::item_instance_rolls::{parse_attributes, roll_modifier_definitions},
};

pub fn roll_gem_instance_effects<R: Rng + ?Sized>(
    gem_data: &'static ItemData,
    gem_instance: &PersistedItemInstance,
    rng: &mut R,
) -> Result<ItemInstanceAttributes, String> {
    let rules = gem_data
        .kind
        .gem_effect_rolls()
        .ok_or_else(|| format!("gem '{}' does not support effect rolls", gem_data.slug))?;

    let mut attributes = parse_attributes(&gem_instance.attributes_json)?;
    if attributes.identified {
        return Err(format!(
            "gem item instance '{}' is already rolled",
            gem_instance.id
        ));
    }

    attributes.identified = true;
    attributes.roll_bias = Some(EquipmentRollBias::Neutral);
    attributes.additional_effects = roll_modifier_definitions(
        rules.roll_count,
        rules.effect_pool,
        EquipmentRollBias::Neutral,
        rng,
    );

    Ok(attributes)
}
