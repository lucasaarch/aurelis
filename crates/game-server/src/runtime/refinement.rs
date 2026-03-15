use rand::Rng;

use crate::resources::internal_api::{InternalApi, PlayableCharacterSnapshot};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefinementOutcome {
    Success {
        old_refinement: i16,
        new_refinement: i16,
    },
    FailedNoChange {
        old_refinement: i16,
    },
    FailedReset {
        old_refinement: i16,
        new_refinement: i16,
    },
}

#[derive(Debug, Clone, Copy)]
struct RefinementRule {
    success_rate_bp: u16,
    reset_on_fail_rate_bp: u16,
}

const REFINEMENT_RULES: [RefinementRule; 7] = [
    RefinementRule {
        success_rate_bp: 10_000,
        reset_on_fail_rate_bp: 0,
    },
    RefinementRule {
        success_rate_bp: 10_000,
        reset_on_fail_rate_bp: 0,
    },
    RefinementRule {
        success_rate_bp: 9500,
        reset_on_fail_rate_bp: 0,
    },
    RefinementRule {
        success_rate_bp: 8500,
        reset_on_fail_rate_bp: 0,
    },
    RefinementRule {
        success_rate_bp: 6500,
        reset_on_fail_rate_bp: 1000,
    },
    RefinementRule {
        success_rate_bp: 4500,
        reset_on_fail_rate_bp: 2000,
    },
    RefinementRule {
        success_rate_bp: 2500,
        reset_on_fail_rate_bp: 3500,
    },
];

pub fn validate_refinement_target(
    snapshot: &PlayableCharacterSnapshot,
    equipment_slot: &str,
) -> Result<(uuid::Uuid, i16), String> {
    let equipped = snapshot
        .equipment
        .iter()
        .find(|entry| entry.slot == equipment_slot)
        .ok_or_else(|| format!("equipment slot '{}' is empty", equipment_slot))?;
    let item_instance = snapshot
        .item_instances
        .iter()
        .find(|instance| instance.id == equipped.item_instance_id)
        .ok_or_else(|| {
            format!(
                "missing equipped item instance '{}'",
                equipped.item_instance_id
            )
        })?;

    if item_instance.refinement >= 7 {
        return Err(format!(
            "item instance '{}' is already at max refinement",
            item_instance.id
        ));
    }

    Ok((item_instance.id, item_instance.refinement.clamp(0, 7)))
}

pub fn roll_refinement_outcome<R: Rng + ?Sized>(
    current_refinement: i16,
    rng: &mut R,
) -> Result<RefinementOutcome, String> {
    let current_refinement = current_refinement.clamp(0, 7);
    if current_refinement >= 7 {
        return Err("item is already at max refinement".to_string());
    }

    let rule = REFINEMENT_RULES
        .get(current_refinement as usize)
        .ok_or_else(|| format!("missing refinement rule for level {}", current_refinement))?;
    let roll = rng.random_range(0..10_000u16);

    if roll < rule.success_rate_bp {
        return Ok(RefinementOutcome::Success {
            old_refinement: current_refinement,
            new_refinement: current_refinement + 1,
        });
    }

    let fail_roll = rng.random_range(0..10_000u16);
    if fail_roll < rule.reset_on_fail_rate_bp {
        return Ok(RefinementOutcome::FailedReset {
            old_refinement: current_refinement,
            new_refinement: 0,
        });
    }

    Ok(RefinementOutcome::FailedNoChange {
        old_refinement: current_refinement,
    })
}

pub fn refine_equipment(
    internal_api: &InternalApi,
    snapshot: &PlayableCharacterSnapshot,
    equipment_slot: &str,
) -> Result<(PlayableCharacterSnapshot, RefinementOutcome), String> {
    let (item_instance_id, current_refinement) =
        validate_refinement_target(snapshot, equipment_slot)?;
    let outcome = roll_refinement_outcome(current_refinement, &mut rand::rng())?;

    match outcome {
        RefinementOutcome::Success { new_refinement, .. }
        | RefinementOutcome::FailedReset { new_refinement, .. } => {
            internal_api.update_item_instance_refinement(
                snapshot.account_id,
                snapshot.character_id,
                item_instance_id,
                new_refinement,
            )?;
        }
        RefinementOutcome::FailedNoChange { .. } => {}
    }

    let reloaded =
        internal_api.load_playable_character(snapshot.account_id, snapshot.character_id)?;
    Ok((reloaded, outcome))
}
