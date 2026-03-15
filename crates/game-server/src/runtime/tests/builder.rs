use uuid::Uuid;

use crate::{
    resources::internal_api::{
        PersistedEquipment, PersistedInventory, PersistedInventoryItem, PersistedItemInstance,
        PersistedItemInstanceGem, PlayableCharacterSnapshot,
    },
    runtime::builder::build_runtime_character,
    runtime::modifier::{ModifierDuration, ModifierSource, RuntimeModifier, StatModifierOp},
    runtime::skill_effects::build_timed_skill_modifier,
    runtime::use_skill::use_skill,
};
use shared::models::{
    character_data::{CharacterSkillUnlocks, CombatAffinity},
    combat_stats::{CombatStats, StatKey},
    reward_stats::RewardStatKey,
    stat_modifier::ModifierStat,
};

#[test]
fn builds_runtime_character_with_base_stats_only() {
    let snapshot = snapshot_base_only();

    let runtime = build_runtime_character(&snapshot).expect("runtime build should succeed");

    assert_eq!(runtime.stats.base.core.hp, 520);
    assert_eq!(runtime.stats.base.core.physical_atk, 48);
    assert_eq!(runtime.stats.from_class.core.hp, 0);
    assert_eq!(runtime.stats.from_equipment.core.hp, 0);
    assert_eq!(
        runtime
            .stats
            .from_persistent_modifiers
            .secondary
            .crit_chance,
        0
    );
    assert_eq!(runtime.combat_affinity, CombatAffinity::Neutral);
    assert_eq!(
        runtime.available_skill_slugs,
        vec!["kael_slash".to_string(), "kael_guarding_strike".to_string(),]
    );
    assert_eq!(runtime.rewards.final_stats.experience_gain, 0);
    assert_eq!(runtime.stats.final_stats.core.hp, 520);
    assert_eq!(runtime.stats.final_stats.core.physical_def, 32);
    assert_eq!(runtime.stats.final_stats.secondary.damage_reduction, 0);
}

#[test]
fn builds_runtime_character_with_class_bonus() {
    let mut snapshot = snapshot_base_only();
    snapshot.current_class_slug = "kael_royal_sentinel".to_string();

    let runtime = build_runtime_character(&snapshot).expect("runtime build should succeed");

    assert_eq!(runtime.stats.base.core.hp, 520);
    assert_eq!(runtime.stats.from_class.core.hp, 320);
    assert_eq!(runtime.stats.from_class.core.physical_atk, 32);
    assert_eq!(runtime.stats.final_stats.core.hp, 840);
    assert_eq!(runtime.stats.final_stats.core.physical_atk, 80);
    assert_eq!(runtime.stats.final_stats.core.atk_spd, 108);
    assert_eq!(runtime.combat_affinity, CombatAffinity::Physical);
    assert_eq!(
        runtime.available_skill_slugs,
        vec![
            "kael_slash".to_string(),
            "kael_guarding_strike".to_string(),
            "sentinel_steel_pulse".to_string(),
        ]
    );
}

#[test]
fn builds_runtime_character_with_fixed_equipment_stats() {
    let mut snapshot = snapshot_base_only();
    snapshot.current_class_slug = "kael_royal_sentinel".to_string();
    snapshot.equipment = vec![
        equipment("weapon", "11111111-1111-1111-1111-111111111111"),
        equipment("chest", "22222222-2222-2222-2222-222222222222"),
        equipment("legs", "33333333-3333-3333-3333-333333333333"),
        equipment("gloves", "44444444-4444-4444-4444-444444444444"),
        equipment("shoes", "55555555-5555-5555-5555-555555555555"),
    ];
    snapshot.item_instances = vec![
        item_instance(
            "11111111-1111-1111-1111-111111111111",
            "kael_training_blade",
        ),
        item_instance(
            "22222222-2222-2222-2222-222222222222",
            "kael_squire_chestplate",
        ),
        item_instance(
            "33333333-3333-3333-3333-333333333333",
            "kael_squire_legguards",
        ),
        item_instance(
            "44444444-4444-4444-4444-444444444444",
            "kael_squire_gauntlets",
        ),
        item_instance("55555555-5555-5555-5555-555555555555", "kael_squire_boots"),
    ];

    let runtime = build_runtime_character(&snapshot).expect("runtime build should succeed");

    assert_eq!(runtime.stats.from_equipment.core.physical_atk, 42);
    assert_eq!(runtime.stats.from_equipment.core.hp, 120);
    assert_eq!(runtime.stats.from_equipment.core.physical_def, 56);
    assert_eq!(runtime.stats.from_equipment.core.magical_def, 29);
    assert_eq!(runtime.stats.from_equipment.core.atk_spd, 7);
    assert_eq!(runtime.stats.from_equipment.core.move_spd, 9);
    assert_eq!(runtime.stats.from_equipment.secondary.damage_reduction, 6);
    assert_eq!(
        runtime
            .stats
            .from_persistent_modifiers
            .secondary
            .crit_chance,
        10
    );
    assert_eq!(
        runtime
            .stats
            .from_persistent_modifiers
            .secondary
            .crit_damage,
        5
    );

    assert_eq!(runtime.stats.final_stats.core.hp, 960);
    assert_eq!(runtime.stats.final_stats.core.physical_atk, 122);
    assert_eq!(runtime.stats.final_stats.core.physical_def, 110);
    assert_eq!(runtime.stats.final_stats.core.magical_def, 51);
    assert_eq!(runtime.stats.final_stats.core.atk_spd, 115);
    assert_eq!(runtime.stats.final_stats.core.move_spd, 114);
    assert_eq!(runtime.stats.final_stats.secondary.damage_reduction, 6);
    assert_eq!(runtime.stats.final_stats.secondary.crit_chance, 10);
    assert_eq!(runtime.stats.final_stats.secondary.crit_damage, 5);
    assert_eq!(
        runtime.available_skill_slugs,
        vec![
            "kael_slash".to_string(),
            "kael_guarding_strike".to_string(),
            "sentinel_steel_pulse".to_string(),
        ]
    );
}

#[test]
fn unlocks_book_gated_skills_by_character_tier_across_lines() {
    let mut snapshot = snapshot_base_only();
    snapshot.current_class_slug = "kael_royal_sentinel".to_string();
    snapshot.level = 15;
    snapshot.skill_unlocks = CharacterSkillUnlocks {
        beginner: true,
        intermediate: false,
    };

    let runtime = build_runtime_character(&snapshot).expect("runtime build should succeed");
    assert!(
        runtime
            .available_skill_slugs
            .contains(&"sentinel_signature_decree".to_string())
    );

    let mut second_line_snapshot = snapshot_base_only();
    second_line_snapshot.current_class_slug = "kael_sovereign_blade".to_string();
    second_line_snapshot.level = 35;
    second_line_snapshot.skill_unlocks = CharacterSkillUnlocks {
        beginner: true,
        intermediate: true,
    };

    let second_line_runtime =
        build_runtime_character(&second_line_snapshot).expect("runtime build should succeed");
    assert!(
        second_line_runtime
            .available_skill_slugs
            .contains(&"ascendant_kingsfall".to_string())
    );
}

#[test]
fn prints_runtime_character_stat_breakdown() {
    let mut snapshot = snapshot_base_only();
    snapshot.current_class_slug = "kael_royal_sentinel".to_string();
    snapshot.equipment = vec![
        equipment("weapon", "11111111-1111-1111-1111-111111111111"),
        equipment("chest", "22222222-2222-2222-2222-222222222222"),
        equipment("legs", "33333333-3333-3333-3333-333333333333"),
        equipment("gloves", "44444444-4444-4444-4444-444444444444"),
        equipment("shoes", "55555555-5555-5555-5555-555555555555"),
    ];
    snapshot.item_instances = vec![
        item_instance(
            "11111111-1111-1111-1111-111111111111",
            "kael_training_blade",
        ),
        item_instance(
            "22222222-2222-2222-2222-222222222222",
            "kael_squire_chestplate",
        ),
        item_instance(
            "33333333-3333-3333-3333-333333333333",
            "kael_squire_legguards",
        ),
        item_instance(
            "44444444-4444-4444-4444-444444444444",
            "kael_squire_gauntlets",
        ),
        item_instance("55555555-5555-5555-5555-555555555555", "kael_squire_boots"),
    ];

    let runtime = build_runtime_character(&snapshot).expect("runtime build should succeed");

    println!(
        "\nRuntime breakdown for {} ({})\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
        runtime.name,
        runtime.current_class_slug,
        format_stats("Base", &runtime.stats.base),
        format_stats("Class", &runtime.stats.from_class),
        format_stats("Equipment", &runtime.stats.from_equipment),
        format_rewards("Reward Base", &runtime.rewards.base),
        format_rewards("Reward Class", &runtime.rewards.from_class),
        format_rewards("Reward Equipment", &runtime.rewards.from_equipment),
        format_stats("Persistent", &runtime.stats.from_persistent_modifiers),
        format_rewards(
            "Reward Persistent",
            &runtime.rewards.from_persistent_modifiers
        ),
        format_stats("Timed", &runtime.stats.from_timed_modifiers),
        format_rewards("Reward Timed", &runtime.rewards.from_timed_modifiers),
        format_stats("Final", &runtime.stats.final_stats),
        format_rewards("Reward Final", &runtime.rewards.final_stats),
    );
}

#[test]
fn applies_persistent_modifiers_to_runtime_stats() {
    let mut snapshot = snapshot_base_only();
    snapshot.current_class_slug = "kael_royal_sentinel".to_string();

    let mut runtime = build_runtime_character(&snapshot).expect("runtime build should succeed");
    runtime.add_persistent_modifier(RuntimeModifier {
        source: ModifierSource::PassiveSkill {
            skill_slug: "focus_of_the_crown".to_string(),
        },
        duration: ModifierDuration::Permanent,
        operations: vec![
            StatModifierOp::AddFlat {
                stat: ModifierStat::Combat(StatKey::Mp),
                value: 100,
            },
            StatModifierOp::AddFlat {
                stat: ModifierStat::Combat(StatKey::CritChance),
                value: 15,
            },
            StatModifierOp::AddPercent {
                stat: ModifierStat::Combat(StatKey::PhysicalAtk),
                value_bp: 1500,
            },
        ],
    });

    assert_eq!(runtime.stats.from_persistent_modifiers.core.mp, 100);
    assert_eq!(
        runtime
            .stats
            .from_persistent_modifiers
            .secondary
            .crit_chance,
        15
    );
    assert_eq!(
        runtime.stats.from_persistent_modifiers.core.physical_atk,
        12
    );
    assert_eq!(runtime.stats.final_stats.core.mp, 380);
    assert_eq!(runtime.stats.final_stats.secondary.crit_chance, 15);
    assert_eq!(runtime.stats.final_stats.core.physical_atk, 92);
}

#[test]
fn applies_reward_modifiers_to_runtime_rewards() {
    let snapshot = snapshot_base_only();

    let mut runtime = build_runtime_character(&snapshot).expect("runtime build should succeed");
    runtime.add_persistent_modifier(RuntimeModifier {
        source: ModifierSource::Debug {
            label: "reward_bonus".to_string(),
        },
        duration: ModifierDuration::Permanent,
        operations: vec![
            StatModifierOp::AddFlat {
                stat: ModifierStat::Reward(RewardStatKey::ExperienceGain),
                value: 2500,
            },
            StatModifierOp::AddFlat {
                stat: ModifierStat::Reward(RewardStatKey::DropRate),
                value: 1200,
            },
            StatModifierOp::AddFlat {
                stat: ModifierStat::Reward(RewardStatKey::CreditGain),
                value: 800,
            },
        ],
    });

    assert_eq!(runtime.stats.final_stats.core.hp, 520);
    assert_eq!(
        runtime.rewards.from_persistent_modifiers.experience_gain,
        2500
    );
    assert_eq!(runtime.rewards.from_persistent_modifiers.drop_rate, 1200);
    assert_eq!(runtime.rewards.from_persistent_modifiers.credit_gain, 800);
    assert_eq!(runtime.rewards.final_stats.experience_gain, 2500);
    assert_eq!(runtime.rewards.final_stats.drop_rate, 1200);
    assert_eq!(runtime.rewards.final_stats.credit_gain, 800);
}

#[test]
fn applies_passive_skill_modifiers_from_catalog_automatically() {
    let mut snapshot = snapshot_base_only();
    snapshot.current_class_slug = "kael_royal_sentinel".to_string();
    snapshot.level = 20;

    let runtime = build_runtime_character(&snapshot).expect("runtime build should succeed");

    assert!(
        runtime
            .available_skill_slugs
            .contains(&"sentinel_discipline".to_string())
    );
    assert_eq!(runtime.stats.from_persistent_modifiers.core.mp, 100);
    assert_eq!(
        runtime
            .stats
            .from_persistent_modifiers
            .secondary
            .crit_chance,
        8
    );
    assert_eq!(runtime.stats.final_stats.core.mp, 380);
    assert_eq!(runtime.stats.final_stats.secondary.crit_chance, 8);
}

#[test]
fn applies_identified_item_instance_attributes_as_persistent_modifiers() {
    let mut snapshot = snapshot_base_only();
    snapshot.current_class_slug = "kael_royal_sentinel".to_string();
    snapshot.equipment = vec![equipment("weapon", "11111111-1111-1111-1111-111111111111")];
    snapshot.item_instances = vec![PersistedItemInstance {
        id: Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap(),
        item_id: Uuid::parse_str("eeeeeeee-eeee-eeee-eeee-eeeeeeeeeeee").unwrap(),
        item_slug: "kael_training_blade".to_string(),
        inventory_type: "equipment".to_string(),
        refinement: 0,
        bonus_gem_slots: 1,
        attributes_json: r#"{
            "identified": true,
            "roll_bias": "physical",
            "reroll_count": 2,
            "additional_effects": [
                { "id": "roll_phys_level", "stat": "physical_attack_level", "kind": "flat", "value": 375 },
                { "id": "roll_crit_damage", "stat": "crit_damage", "kind": "flat", "value": 5 },
                { "id": "roll_phys_atk_pct", "stat": "physical_atk", "kind": "percent", "value": 1000 }
            ]
        }"#
        .to_string(),
        in_shared_storage: false,
        in_trade: false,
        gems: Vec::<PersistedItemInstanceGem>::new(),
    }];

    let runtime = build_runtime_character(&snapshot).expect("runtime build should succeed");

    assert_eq!(runtime.stats.from_equipment.core.physical_atk, 42);
    assert_eq!(runtime.stats.from_equipment.secondary.crit_damage, 5);
    assert_eq!(
        runtime.stats.from_equipment.secondary.physical_attack_level,
        375
    );
    assert_eq!(
        runtime
            .stats
            .from_persistent_modifiers
            .secondary
            .crit_chance,
        10
    );
    assert_eq!(
        runtime
            .stats
            .from_persistent_modifiers
            .secondary
            .crit_damage,
        5
    );
    assert_eq!(
        runtime.stats.from_persistent_modifiers.core.physical_atk,
        12
    );
    assert_eq!(runtime.stats.final_stats.core.physical_atk, 134);
    assert_eq!(runtime.stats.final_stats.secondary.crit_chance, 10);
    assert_eq!(runtime.stats.final_stats.secondary.crit_damage, 10);
    assert_eq!(
        runtime.stats.final_stats.secondary.physical_attack_level,
        375
    );
}

#[test]
fn applies_refinement_to_equipped_item_instance_stats() {
    let mut snapshot = snapshot_base_only();
    snapshot.current_class_slug = "kael_royal_sentinel".to_string();
    snapshot.equipment = vec![equipment("weapon", "11111111-1111-1111-1111-111111111111")];
    let mut blade = item_instance(
        "11111111-1111-1111-1111-111111111111",
        "kael_training_blade",
    );
    blade.refinement = 3;
    snapshot.item_instances = vec![blade];

    let runtime = build_runtime_character(&snapshot).expect("runtime build should succeed");

    assert_eq!(runtime.stats.from_equipment.core.physical_atk, 54);
    assert_eq!(runtime.stats.final_stats.core.physical_atk, 134);
    assert_eq!(runtime.stats.final_stats.secondary.crit_chance, 10);
}

#[test]
fn clamps_refinement_at_plus_seven() {
    let mut snapshot = snapshot_base_only();
    snapshot.current_class_slug = "kael_royal_sentinel".to_string();
    snapshot.equipment = vec![equipment("weapon", "11111111-1111-1111-1111-111111111111")];
    let mut blade = item_instance(
        "11111111-1111-1111-1111-111111111111",
        "kael_training_blade",
    );
    blade.refinement = 99;
    snapshot.item_instances = vec![blade];

    let runtime = build_runtime_character(&snapshot).expect("runtime build should succeed");

    assert_eq!(runtime.stats.from_equipment.core.physical_atk, 71);
    assert_eq!(runtime.stats.final_stats.core.physical_atk, 151);
}

#[test]
fn applies_gems_inside_equipped_item_instance_calculation() {
    let mut snapshot = snapshot_base_only();
    snapshot.current_class_slug = "kael_royal_sentinel".to_string();
    snapshot.equipment = vec![equipment("weapon", "11111111-1111-1111-1111-111111111111")];

    let mut blade = item_instance(
        "11111111-1111-1111-1111-111111111111",
        "kael_training_blade",
    );
    blade.gems = vec![
        PersistedItemInstanceGem {
            slot_index: 0,
            gem_instance_id: Uuid::parse_str("66666666-6666-6666-6666-666666666666").unwrap(),
        },
        PersistedItemInstanceGem {
            slot_index: 1,
            gem_instance_id: Uuid::parse_str("77777777-7777-7777-7777-777777777777").unwrap(),
        },
    ];
    snapshot.item_instances = vec![
        blade,
        item_instance("66666666-6666-6666-6666-666666666666", "vitality_gem"),
        item_instance("77777777-7777-7777-7777-777777777777", "fury_gem"),
    ];

    let runtime = build_runtime_character(&snapshot).expect("runtime build should succeed");

    assert_eq!(runtime.stats.from_equipment.core.physical_atk, 42);
    assert_eq!(runtime.stats.from_equipment.core.hp, 60);
    assert_eq!(runtime.stats.from_equipment.secondary.crit_damage, 4);
    assert_eq!(runtime.stats.final_stats.core.hp, 900);
    assert_eq!(runtime.stats.final_stats.secondary.crit_damage, 9);
}

#[test]
fn applies_rolled_gem_instance_effects_inside_equipped_item_instance_calculation() {
    let mut snapshot = snapshot_base_only();
    snapshot.current_class_slug = "kael_royal_sentinel".to_string();
    snapshot.equipment = vec![equipment("weapon", "11111111-1111-1111-1111-111111111111")];

    let mut blade = item_instance(
        "11111111-1111-1111-1111-111111111111",
        "kael_training_blade",
    );
    blade.gems = vec![PersistedItemInstanceGem {
        slot_index: 0,
        gem_instance_id: Uuid::parse_str("88888888-8888-8888-8888-888888888888").unwrap(),
    }];

    let mut chaos_gem = item_instance("88888888-8888-8888-8888-888888888888", "chaos_gem");
    chaos_gem.attributes_json = r#"{
        "identified": true,
        "roll_bias": "neutral",
        "reroll_count": 0,
        "additional_effects": [
            { "id": "gem_roll_hp", "stat": "hp", "kind": "flat", "value": 72 }
        ]
    }"#
    .to_string();

    snapshot.item_instances = vec![blade, chaos_gem];

    let runtime = build_runtime_character(&snapshot).expect("runtime build should succeed");

    assert_eq!(runtime.stats.from_equipment.core.hp, 72);
    assert_eq!(runtime.stats.final_stats.core.hp, 912);
}

#[test]
fn applies_and_expires_timed_modifiers() {
    let mut snapshot = snapshot_base_only();
    snapshot.current_class_slug = "kael_royal_sentinel".to_string();

    let mut runtime = build_runtime_character(&snapshot).expect("runtime build should succeed");
    runtime.add_timed_modifier(RuntimeModifier {
        source: ModifierSource::ActiveBuff {
            effect_slug: "battle_hymn".to_string(),
        },
        duration: ModifierDuration::Timed {
            remaining_ms: 15_000,
        },
        operations: vec![
            StatModifierOp::AddPercent {
                stat: ModifierStat::Combat(StatKey::PhysicalAtk),
                value_bp: 1500,
            },
            StatModifierOp::AddPercent {
                stat: ModifierStat::Combat(StatKey::MagicalAtk),
                value_bp: 1500,
            },
        ],
    });

    assert_eq!(runtime.stats.from_timed_modifiers.core.physical_atk, 12);
    assert_eq!(runtime.stats.from_timed_modifiers.core.magical_atk, 1);
    assert_eq!(runtime.stats.final_stats.core.physical_atk, 92);
    assert_eq!(runtime.stats.final_stats.core.magical_atk, 13);

    let changed = runtime.tick_timed_modifiers(15_000);
    assert!(changed);
    assert_eq!(runtime.stats.from_timed_modifiers.core.physical_atk, 0);
    assert_eq!(runtime.stats.from_timed_modifiers.core.magical_atk, 0);
    assert_eq!(runtime.stats.final_stats.core.physical_atk, 80);
    assert_eq!(runtime.stats.final_stats.core.magical_atk, 12);
}

#[test]
fn builds_advantage_skill_buff_from_catalog() {
    let mut snapshot = snapshot_base_only();
    snapshot.current_class_slug = "kael_royal_sentinel".to_string();
    snapshot.level = 20;

    let mut runtime = build_runtime_character(&snapshot).expect("runtime build should succeed");
    let modifier =
        build_timed_skill_modifier("kael", "sentinel_steel_pulse").expect("buff should build");

    runtime.add_timed_modifier(modifier);

    assert_eq!(runtime.stats.from_timed_modifiers.core.physical_atk, 12);
    assert_eq!(runtime.stats.final_stats.core.physical_atk, 92);
}

#[test]
fn uses_advantage_skill_and_spends_mp() {
    let mut snapshot = snapshot_base_only();
    snapshot.current_class_slug = "kael_royal_sentinel".to_string();
    snapshot.level = 20;

    let mut runtime = build_runtime_character(&snapshot).expect("runtime build should succeed");
    let initial_mp = runtime.resources.current_mp;

    use_skill(&mut runtime, "sentinel_steel_pulse").expect("skill use should succeed");

    assert_eq!(runtime.resources.current_mp, initial_mp - 36);
    assert_eq!(runtime.stats.from_timed_modifiers.core.physical_atk, 12);
    assert_eq!(runtime.stats.final_stats.core.physical_atk, 92);
    assert_eq!(
        runtime
            .skill_cooldowns_ms
            .get("sentinel_steel_pulse")
            .copied(),
        Some(12_000)
    );
}

#[test]
fn rejects_skill_use_when_mp_is_insufficient() {
    let mut snapshot = snapshot_base_only();
    snapshot.current_class_slug = "kael_royal_sentinel".to_string();
    snapshot.level = 20;

    let mut runtime = build_runtime_character(&snapshot).expect("runtime build should succeed");
    runtime.resources.current_mp = 10;

    let error = use_skill(&mut runtime, "sentinel_steel_pulse")
        .expect_err("skill use should fail without enough MP");

    assert_eq!(error, "not enough MP");
    assert_eq!(runtime.stats.from_timed_modifiers.core.physical_atk, 0);
}

#[test]
fn expires_advantage_skill_buff_after_duration() {
    let mut snapshot = snapshot_base_only();
    snapshot.current_class_slug = "kael_royal_sentinel".to_string();
    snapshot.level = 20;

    let mut runtime = build_runtime_character(&snapshot).expect("runtime build should succeed");
    use_skill(&mut runtime, "sentinel_steel_pulse").expect("skill use should succeed");

    let changed = runtime.tick_timed_modifiers(15_000);

    assert!(changed);
    assert_eq!(runtime.stats.from_timed_modifiers.core.physical_atk, 0);
    assert_eq!(runtime.stats.final_stats.core.physical_atk, 80);
}

#[test]
fn rejects_skill_use_while_on_cooldown() {
    let mut snapshot = snapshot_base_only();
    snapshot.current_class_slug = "kael_royal_sentinel".to_string();
    snapshot.level = 20;

    let mut runtime = build_runtime_character(&snapshot).expect("runtime build should succeed");
    use_skill(&mut runtime, "sentinel_steel_pulse").expect("first use should succeed");

    let error =
        use_skill(&mut runtime, "sentinel_steel_pulse").expect_err("second use should fail");

    assert_eq!(error, "skill 'sentinel_steel_pulse' is on cooldown");
}

#[test]
fn expires_skill_cooldown_after_duration() {
    let mut snapshot = snapshot_base_only();
    snapshot.current_class_slug = "kael_royal_sentinel".to_string();
    snapshot.level = 20;

    let mut runtime = build_runtime_character(&snapshot).expect("runtime build should succeed");
    use_skill(&mut runtime, "sentinel_steel_pulse").expect("first use should succeed");

    let changed = runtime.tick_skill_cooldowns(12_000);

    assert!(changed);
    assert!(!runtime.is_skill_on_cooldown("sentinel_steel_pulse"));
}

fn snapshot_base_only() -> PlayableCharacterSnapshot {
    PlayableCharacterSnapshot {
        account_id: Uuid::parse_str("aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa").unwrap(),
        character_id: Uuid::parse_str("bbbbbbbb-bbbb-bbbb-bbbb-bbbbbbbbbbbb").unwrap(),
        name: "Kaelzinho".to_string(),
        base_character_slug: "kael".to_string(),
        current_class_slug: "kael".to_string(),
        level: 12,
        experience: 8450,
        credits: 3200,
        skill_unlocks: CharacterSkillUnlocks::default(),
        inventories: vec![PersistedInventory {
            id: Uuid::parse_str("cccccccc-cccc-cccc-cccc-cccccccccccc").unwrap(),
            inventory_type: "equipment".to_string(),
            capacity: 56,
            items: vec![PersistedInventoryItem {
                id: Uuid::parse_str("dddddddd-dddd-dddd-dddd-dddddddddddd").unwrap(),
                inventory_id: Uuid::parse_str("cccccccc-cccc-cccc-cccc-cccccccccccc").unwrap(),
                inventory_type: "equipment".to_string(),
                slot_index: 0,
                quantity: 1,
                item_instance_id: None,
                item_id: None,
                item_slug: None,
            }],
        }],
        equipment: vec![],
        item_instances: vec![],
    }
}

fn equipment(slot: &str, item_instance_id: &str) -> PersistedEquipment {
    PersistedEquipment {
        slot: slot.to_string(),
        item_instance_id: Uuid::parse_str(item_instance_id).unwrap(),
    }
}

fn item_instance(id: &str, item_slug: &str) -> PersistedItemInstance {
    PersistedItemInstance {
        id: Uuid::parse_str(id).unwrap(),
        item_id: Uuid::parse_str("eeeeeeee-eeee-eeee-eeee-eeeeeeeeeeee").unwrap(),
        item_slug: item_slug.to_string(),
        inventory_type: "equipment".to_string(),
        refinement: 0,
        bonus_gem_slots: 0,
        attributes_json: "{}".to_string(),
        in_shared_storage: false,
        in_trade: false,
        gems: Vec::<PersistedItemInstanceGem>::new(),
    }
}

fn format_stats(label: &str, stats: &CombatStats) -> String {
    format!(
        "{label}:\n  hp={}\n  mp={}\n  physical_atk={}\n  magical_atk={}\n  physical_def={}\n  magical_def={}\n  move_spd={}\n  atk_spd={}\n  damage_reduction={}\n  crit_chance={}\n  crit_damage={}\n  accuracy={}\n  physical_attack_level={}\n  magical_attack_level={}\n  physical_pen={}\n  magical_pen={}\n  hp_regen={}\n  mp_regen={}\n  life_steal={}\n  cooldown_reduction={}\n  crit_resistance={}\n  knockback_resistance={}\n  cc_resistance={}",
        stats.core.hp,
        stats.core.mp,
        stats.core.physical_atk,
        stats.core.magical_atk,
        stats.core.physical_def,
        stats.core.magical_def,
        stats.core.move_spd,
        stats.core.atk_spd,
        stats.secondary.damage_reduction,
        stats.secondary.crit_chance,
        stats.secondary.crit_damage,
        stats.secondary.accuracy,
        stats.secondary.physical_attack_level,
        stats.secondary.magical_attack_level,
        stats.secondary.physical_pen,
        stats.secondary.magical_pen,
        stats.secondary.hp_regen,
        stats.secondary.mp_regen,
        stats.secondary.life_steal,
        stats.secondary.cooldown_reduction,
        stats.secondary.crit_resistance,
        stats.secondary.knockback_resistance,
        stats.secondary.cc_resistance,
    )
}

fn format_rewards(label: &str, stats: &shared::models::reward_stats::RewardStats) -> String {
    format!(
        "{label}:\n  experience_gain={}\n  drop_rate={}\n  credit_gain={}",
        stats.experience_gain, stats.drop_rate, stats.credit_gain,
    )
}
