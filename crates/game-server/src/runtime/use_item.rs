use rand::Rng;
use shared::{
    data::cities::find_item_by_slug,
    models::{
        item_data::{ItemData, ItemKind, SpecialEffect},
        skill_data::CharacterSkillUnlockTier,
    },
};
use uuid::Uuid;

use crate::{
    resources::internal_api::{InternalApi, PersistedItemInstance, PlayableCharacterSnapshot},
    runtime::{
        builder::build_runtime_character,
        character::RuntimeCharacter,
        item_instance_rolls::{parse_attributes, roll_modifier_definitions},
    },
};

pub trait UseItemApi {
    fn unlock_character_skill_tier(
        &self,
        account_id: Uuid,
        character_id: Uuid,
        tier: CharacterSkillUnlockTier,
    ) -> Result<(), String>;

    fn persist_item_instance_state(
        &self,
        account_id: Uuid,
        character_id: Uuid,
        item_instance_id: Uuid,
        bonus_gem_slots: i16,
        attributes_json: String,
    ) -> Result<(), String>;

    fn consume_inventory_item(
        &self,
        account_id: Uuid,
        character_id: Uuid,
        inventory_type: String,
        slot: i16,
        quantity: i16,
    ) -> Result<(), String>;

    fn load_playable_character(
        &self,
        account_id: Uuid,
        character_id: Uuid,
    ) -> Result<PlayableCharacterSnapshot, String>;
}

impl UseItemApi for InternalApi {
    fn unlock_character_skill_tier(
        &self,
        account_id: Uuid,
        character_id: Uuid,
        tier: CharacterSkillUnlockTier,
    ) -> Result<(), String> {
        Self::unlock_character_skill_tier(self, account_id, character_id, tier)
    }

    fn persist_item_instance_state(
        &self,
        account_id: Uuid,
        character_id: Uuid,
        item_instance_id: Uuid,
        bonus_gem_slots: i16,
        attributes_json: String,
    ) -> Result<(), String> {
        Self::persist_item_instance_state(
            self,
            account_id,
            character_id,
            item_instance_id,
            bonus_gem_slots,
            attributes_json,
        )
    }

    fn consume_inventory_item(
        &self,
        account_id: Uuid,
        character_id: Uuid,
        inventory_type: String,
        slot: i16,
        quantity: i16,
    ) -> Result<(), String> {
        Self::consume_inventory_item(
            self,
            account_id,
            character_id,
            inventory_type,
            slot,
            quantity,
        )
    }

    fn load_playable_character(
        &self,
        account_id: Uuid,
        character_id: Uuid,
    ) -> Result<PlayableCharacterSnapshot, String> {
        Self::load_playable_character(self, account_id, character_id)
    }
}

pub struct UseItemRequest<'a, A: UseItemApi> {
    pub api: &'a A,
    pub snapshot: &'a PlayableCharacterSnapshot,
    pub inventory_type: &'a str,
    pub slot: i16,
    pub target_equipment_slot: Option<&'a str>,
}

pub struct UseItemResult {
    pub snapshot: PlayableCharacterSnapshot,
    pub runtime_character: RuntimeCharacter,
}

pub fn use_item<A: UseItemApi>(request: UseItemRequest<'_, A>) -> Result<UseItemResult, String> {
    use_item_with_rng(request, &mut rand::rng())
}

pub fn use_item_with_rng<A: UseItemApi, R: Rng + ?Sized>(
    request: UseItemRequest<'_, A>,
    rng: &mut R,
) -> Result<UseItemResult, String> {
    let inventory = request
        .snapshot
        .inventories
        .iter()
        .find(|inventory| inventory.inventory_type == request.inventory_type)
        .ok_or_else(|| format!("inventory '{}' does not exist", request.inventory_type))?;

    let inventory_item = inventory
        .items
        .iter()
        .find(|item| item.slot_index == request.slot)
        .ok_or_else(|| "inventory slot is empty".to_string())?;

    let item_slug = inventory_item
        .item_slug
        .as_deref()
        .ok_or_else(|| "inventory item has no catalog slug".to_string())?;
    let item_data =
        find_item_by_slug(item_slug).ok_or_else(|| format!("unknown item slug '{}'", item_slug))?;

    match &item_data.kind {
        ItemKind::Special(special) => match special.effect {
            SpecialEffect::CharacterSkillUnlock { tier } => {
                use_character_skill_unlock_item(request, tier)
            }
            SpecialEffect::EquipmentReroll => {
                use_equipment_reroll_scroll(request, inventory_item, item_data, rng)
            }
            _ => Err(format!("item '{}' is not usable yet", item_slug)),
        },
        _ => Err(format!("item '{}' is not usable", item_slug)),
    }
}

fn use_character_skill_unlock_item<A: UseItemApi>(
    request: UseItemRequest<'_, A>,
    tier: CharacterSkillUnlockTier,
) -> Result<UseItemResult, String> {
    let already_unlocked = match tier {
        CharacterSkillUnlockTier::Beginner => request.snapshot.skill_unlocks.beginner,
        CharacterSkillUnlockTier::Intermediate => request.snapshot.skill_unlocks.intermediate,
    };
    if already_unlocked {
        return Err("character already unlocked this skill tier".to_string());
    }

    let required_level = match tier {
        CharacterSkillUnlockTier::Beginner => 15,
        CharacterSkillUnlockTier::Intermediate => 35,
    };
    if request.snapshot.level < required_level {
        return Err(format!(
            "character level {} is below required level {}",
            request.snapshot.level, required_level
        ));
    }

    request.api.unlock_character_skill_tier(
        request.snapshot.account_id,
        request.snapshot.character_id,
        tier,
    )?;
    request.api.consume_inventory_item(
        request.snapshot.account_id,
        request.snapshot.character_id,
        request.inventory_type.to_string(),
        request.slot,
        1,
    )?;

    reload_runtime(request.api, request.snapshot)
}

fn use_equipment_reroll_scroll<A: UseItemApi, R: Rng + ?Sized>(
    request: UseItemRequest<'_, A>,
    scroll_inventory_item: &crate::resources::internal_api::PersistedInventoryItem,
    _item_data: &'static ItemData,
    rng: &mut R,
) -> Result<UseItemResult, String> {
    let equipment_slot = request
        .target_equipment_slot
        .ok_or_else(|| "item requires a target equipment slot".to_string())?;
    let equipped = request
        .snapshot
        .equipment
        .iter()
        .find(|equipment| equipment.slot == equipment_slot)
        .ok_or_else(|| format!("equipment slot '{}' is empty", equipment_slot))?;
    let item_instance = request
        .snapshot
        .item_instances
        .iter()
        .find(|item| item.id == equipped.item_instance_id)
        .ok_or_else(|| "equipped item instance is missing from snapshot".to_string())?;
    let item_data = find_item_by_slug(&item_instance.item_slug)
        .ok_or_else(|| format!("unknown item slug '{}'", item_instance.item_slug))?;
    let rules = item_data
        .kind
        .identification()
        .ok_or_else(|| format!("item '{}' cannot be rerolled", item_data.slug))?;
    if rules.additional_effect_count <= 0 || rules.additional_effect_pool.is_empty() {
        return Err(format!(
            "item '{}' has no rerollable additional effects",
            item_data.slug
        ));
    }

    let updated_attributes = reroll_equipment_item_instance(item_instance, item_data, rng)?;
    let serialized = serde_json::to_string(&updated_attributes)
        .map_err(|err| format!("failed to serialize rerolled attributes: {err}"))?;

    request.api.persist_item_instance_state(
        request.snapshot.account_id,
        request.snapshot.character_id,
        item_instance.id,
        item_instance.bonus_gem_slots,
        serialized,
    )?;
    request.api.consume_inventory_item(
        request.snapshot.account_id,
        request.snapshot.character_id,
        scroll_inventory_item.inventory_type.clone(),
        scroll_inventory_item.slot_index,
        1,
    )?;

    reload_runtime(request.api, request.snapshot)
}

fn reroll_equipment_item_instance<R: Rng + ?Sized>(
    item_instance: &PersistedItemInstance,
    item_data: &'static ItemData,
    rng: &mut R,
) -> Result<shared::models::item_instance_attributes::ItemInstanceAttributes, String> {
    let rules = item_data
        .kind
        .identification()
        .ok_or_else(|| format!("item '{}' cannot be rerolled", item_data.slug))?;

    let mut attributes = parse_attributes(&item_instance.attributes_json)?;
    if !attributes.identified {
        return Err("item must be identified before rerolling".to_string());
    }

    let bias = attributes.roll_bias.unwrap_or(rules.bias);
    attributes.roll_bias = Some(bias);
    attributes.additional_effects = roll_modifier_definitions(
        rules.additional_effect_count,
        rules.additional_effect_pool,
        bias,
        rng,
    );
    attributes.reroll_count = attributes.reroll_count.saturating_add(1);

    Ok(attributes)
}

fn reload_runtime<A: UseItemApi>(
    api: &A,
    snapshot: &PlayableCharacterSnapshot,
) -> Result<UseItemResult, String> {
    let reloaded = api.load_playable_character(snapshot.account_id, snapshot.character_id)?;
    let runtime_character = build_runtime_character(&reloaded)?;

    Ok(UseItemResult {
        snapshot: reloaded,
        runtime_character,
    })
}
