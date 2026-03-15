use shared::{
    data::characters::find_skill_by_slug,
    models::skill_data::{SkillData, SkillKind},
};

use crate::runtime::modifier::{ModifierDuration, ModifierSource, RuntimeModifier, StatModifierOp};

pub fn build_passive_skill_modifiers(skills: &[&'static SkillData]) -> Vec<RuntimeModifier> {
    skills
        .iter()
        .filter(|skill| matches!(skill.kind, SkillKind::Passive))
        .flat_map(|skill| {
            skill
                .passive_modifiers
                .iter()
                .map(|modifier| RuntimeModifier {
                    source: ModifierSource::PassiveSkill {
                        skill_slug: skill.slug.to_string(),
                    },
                    duration: ModifierDuration::Permanent,
                    operations: vec![match modifier.kind {
                        shared::models::item_instance_attributes::StatModifierValueKind::Flat => {
                            StatModifierOp::AddFlat {
                                stat: modifier.stat,
                                value: modifier.value,
                            }
                        }
                        shared::models::item_instance_attributes::StatModifierValueKind::Percent => {
                            StatModifierOp::AddPercent {
                                stat: modifier.stat,
                                value_bp: modifier.value,
                            }
                        }
                    }],
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn build_timed_skill_modifier(
    character_slug: &str,
    skill_slug: &str,
) -> Result<RuntimeModifier, String> {
    let skill = find_skill_by_slug(character_slug, skill_slug)
        .ok_or_else(|| format!("unknown skill '{}'", skill_slug))?;

    let timed_buff = skill
        .timed_buff
        .as_ref()
        .ok_or_else(|| format!("skill '{}' has no timed buff effect", skill_slug))?;

    Ok(RuntimeModifier {
        source: ModifierSource::ActiveBuff {
            effect_slug: skill.slug.to_string(),
        },
        duration: ModifierDuration::Timed {
            remaining_ms: timed_buff.duration_ms,
        },
        operations: timed_buff
            .modifiers
            .iter()
            .map(|modifier| match modifier.kind {
                shared::models::item_instance_attributes::StatModifierValueKind::Flat => {
                    StatModifierOp::AddFlat {
                        stat: modifier.stat,
                        value: modifier.value,
                    }
                }
                shared::models::item_instance_attributes::StatModifierValueKind::Percent => {
                    StatModifierOp::AddPercent {
                        stat: modifier.stat,
                        value_bp: modifier.value,
                    }
                }
            })
            .collect(),
    })
}
