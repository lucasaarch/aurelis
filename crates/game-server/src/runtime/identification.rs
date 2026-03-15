use rand::Rng;
use shared::models::{
    item_data::{CatalogStatModifierDefinition, EquipmentIdentificationRules, ItemData},
    item_instance_attributes::{
        EquipmentRollBias, ItemInstanceAttributes, ItemInstanceStatModifier,
    },
};

use crate::resources::internal_api::PersistedItemInstance;

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
    attributes.additional_effects = roll_additional_effects(rules, rng)
        .into_iter()
        .collect::<Vec<_>>();

    Ok(attributes)
}

fn parse_attributes(json: &str) -> Result<ItemInstanceAttributes, String> {
    if json.trim().is_empty() || json.trim() == "{}" {
        return Ok(ItemInstanceAttributes::default());
    }

    serde_json::from_str(json)
        .map_err(|err| format!("invalid item instance attributes json: {err}"))
}

fn roll_additional_effects<R: Rng + ?Sized>(
    rules: EquipmentIdentificationRules,
    rng: &mut R,
) -> Vec<ItemInstanceStatModifier> {
    let mut available = rules.additional_effect_pool.to_vec();
    let mut rolled = Vec::new();

    let count = usize::try_from(rules.additional_effect_count.max(0)).unwrap_or(0);
    for _ in 0..count.min(available.len()) {
        let index = choose_weighted_index(&available, rules.bias, rng);
        let definition = available.swap_remove(index);
        let value = if definition.min_value == definition.max_value {
            definition.min_value
        } else {
            rng.random_range(definition.min_value..=definition.max_value)
        };

        rolled.push(ItemInstanceStatModifier {
            id: definition.id.to_string(),
            stat: definition.stat,
            kind: definition.kind,
            value,
        });
    }

    rolled
}

fn choose_weighted_index<R: Rng + ?Sized>(
    pool: &[CatalogStatModifierDefinition],
    bias: EquipmentRollBias,
    rng: &mut R,
) -> usize {
    let weights = pool
        .iter()
        .map(|definition| adjusted_weight(definition, bias))
        .collect::<Vec<_>>();
    let total_weight: u32 = weights.iter().sum();
    if total_weight == 0 {
        return 0;
    }

    let mut roll = rng.random_range(0..total_weight);
    for (index, weight) in weights.iter().enumerate() {
        if roll < *weight {
            return index;
        }
        roll -= *weight;
    }

    pool.len().saturating_sub(1)
}

fn adjusted_weight(definition: &CatalogStatModifierDefinition, bias: EquipmentRollBias) -> u32 {
    let base = u32::from(definition.weight.max(1));
    match (
        bias,
        definition.id.contains("physical"),
        definition.id.contains("magical"),
    ) {
        (EquipmentRollBias::Physical, true, false) => base.saturating_mul(2),
        (EquipmentRollBias::Physical, false, true) => base.max(1) / 2 + 1,
        (EquipmentRollBias::Magical, false, true) => base.saturating_mul(2),
        (EquipmentRollBias::Magical, true, false) => base.max(1) / 2 + 1,
        _ => base,
    }
}
