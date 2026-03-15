use shared::{
    data::cities::find_item_by_slug,
    models::{
        item_data::{ItemKind, SpecialEffect},
        skill_data::CharacterSkillUnlockTier,
    },
};

use crate::{
    resources::internal_api::{InternalApi, PlayableCharacterSnapshot},
    runtime::{builder::build_runtime_character, character::RuntimeCharacter},
};

pub struct UseItemRequest<'a> {
    pub internal_api: &'a InternalApi,
    pub snapshot: &'a PlayableCharacterSnapshot,
    pub inventory_type: &'a str,
    pub slot: i16,
}

pub struct UseItemResult {
    pub runtime_character: RuntimeCharacter,
}

pub fn use_item(request: UseItemRequest<'_>) -> Result<UseItemResult, String> {
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
            _ => Err(format!("item '{}' is not usable yet", item_slug)),
        },
        _ => Err(format!("item '{}' is not usable", item_slug)),
    }
}

fn use_character_skill_unlock_item(
    request: UseItemRequest<'_>,
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

    request.internal_api.unlock_character_skill_tier(
        request.snapshot.account_id,
        request.snapshot.character_id,
        tier,
    )?;
    request.internal_api.consume_inventory_item(
        request.snapshot.account_id,
        request.snapshot.character_id,
        request.inventory_type.to_string(),
        request.slot,
        1,
    )?;

    let reloaded = request
        .internal_api
        .load_playable_character(request.snapshot.account_id, request.snapshot.character_id)?;
    let runtime_character = build_runtime_character(&reloaded)?;

    Ok(UseItemResult { runtime_character })
}
