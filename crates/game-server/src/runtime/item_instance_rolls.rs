use rand::Rng;
use shared::models::{
    item_data::CatalogStatModifierDefinition,
    item_instance_attributes::{
        EquipmentRollBias, ItemInstanceAttributes, ItemInstanceStatModifier,
    },
};

pub fn parse_attributes(json: &str) -> Result<ItemInstanceAttributes, String> {
    if json.trim().is_empty() || json.trim() == "{}" {
        return Ok(ItemInstanceAttributes::default());
    }

    serde_json::from_str(json)
        .map_err(|err| format!("invalid item instance attributes json: {err}"))
}

pub fn roll_modifier_definitions<R: Rng + ?Sized>(
    count: i16,
    pool: &'static [CatalogStatModifierDefinition],
    bias: EquipmentRollBias,
    rng: &mut R,
) -> Vec<ItemInstanceStatModifier> {
    let mut available = pool.to_vec();
    let mut rolled = Vec::new();

    let count = usize::try_from(count.max(0)).unwrap_or(0);
    for _ in 0..count.min(available.len()) {
        let index = choose_weighted_index(&available, bias, rng);
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
