// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "character_class"))]
    pub struct CharacterClass;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "character_location"))]
    pub struct CharacterLocation;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "currency_origin"))]
    pub struct CurrencyOrigin;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "equipment_slot_type"))]
    pub struct EquipmentSlotType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "inventory_type"))]
    pub struct InventoryType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "item_rarity"))]
    pub struct ItemRarity;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "mob_type"))]
    pub struct MobType;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "quest_status"))]
    pub struct QuestStatus;
}

diesel::table! {
    accounts (id) {
        id -> Uuid,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        max_characters -> Int2,
        shared_storage_enabled -> Bool,
        shared_storage_capacity -> Int2,
        cash -> Int8,
        stored_credits -> Int8,
        is_admin -> Bool,
        god_mode -> Bool,
        email_verified -> Bool,
        email_verified_at -> Nullable<Timestamptz>,
        #[max_length = 255]
        email_verify_token -> Nullable<Varchar>,
        email_verify_token_expires -> Nullable<Timestamptz>,
        #[max_length = 255]
        password_reset_token -> Nullable<Varchar>,
        password_reset_expires -> Nullable<Timestamptz>,
        banned_at -> Nullable<Timestamptz>,
        banned_reason -> Nullable<Text>,
        suspended_until -> Nullable<Timestamptz>,
        chat_restricted_until -> Nullable<Timestamptz>,
        last_login_at -> Nullable<Timestamptz>,
        #[max_length = 45]
        last_login_ip -> Nullable<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        deleted_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    character_consumable_slots (character_id, slot) {
        character_id -> Uuid,
        slot -> Int2,
        item_instance_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    character_evolution (character_id) {
        character_id -> Uuid,
        line_id -> Nullable<Uuid>,
        current_step -> Int2,
        last_evolved_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::QuestStatus;

    character_quests (id) {
        id -> Uuid,
        character_id -> Uuid,
        quest_id -> Uuid,
        status -> QuestStatus,
        started_at -> Nullable<Timestamptz>,
        completed_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    character_skill_slots (character_id, slot) {
        character_id -> Uuid,
        slot -> Int2,
        skill_id -> Uuid,
    }
}

diesel::table! {
    character_skills (character_id, skill_id) {
        character_id -> Uuid,
        skill_id -> Uuid,
        unlocked_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::CharacterClass;
    use super::sql_types::CharacterLocation;

    characters (id) {
        id -> Uuid,
        account_id -> Uuid,
        #[max_length = 24]
        name -> Varchar,
        class -> CharacterClass,
        level -> Int2,
        experience -> Int8,
        location -> CharacterLocation,
        credits -> Int8,
        equipment_inventory_capacity -> Int2,
        accessory_inventory_capacity -> Int2,
        consumable_inventory_capacity -> Int2,
        material_inventory_capacity -> Int2,
        quest_item_inventory_capacity -> Int2,
        special_inventory_capacity -> Int2,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::CurrencyOrigin;

    currency_transactions (id) {
        id -> Uuid,
        account_id -> Nullable<Uuid>,
        character_id -> Nullable<Uuid>,
        #[max_length = 16]
        currency -> Varchar,
        amount -> Int8,
        balance_after -> Int8,
        origin -> CurrencyOrigin,
        reference_id -> Nullable<Uuid>,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    dungeon_history (id) {
        id -> Uuid,
        character_id -> Uuid,
        #[max_length = 32]
        dungeon_id -> Varchar,
        hard_mode -> Bool,
        completed_at -> Timestamptz,
        xp_gained -> Int4,
        duration_secs -> Nullable<Int4>,
    }
}

diesel::table! {
    dungeon_mobs (id) {
        id -> Uuid,
        mob_id -> Uuid,
        #[max_length = 32]
        dungeon_id -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::EquipmentSlotType;

    equipment (character_id, slot) {
        character_id -> Uuid,
        slot -> EquipmentSlotType,
        item_instance_id -> Uuid,
        equipped_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::CharacterClass;

    evolution_lines (id) {
        id -> Uuid,
        character_class -> CharacterClass,
        #[max_length = 64]
        name -> Varchar,
        description -> Nullable<Text>,
        order_index -> Int2,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    evolution_steps (id) {
        id -> Uuid,
        line_id -> Uuid,
        #[max_length = 64]
        name -> Varchar,
        description -> Nullable<Text>,
        step_index -> Int2,
        level_req -> Int2,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::InventoryType;

    inventory (id) {
        id -> Uuid,
        character_id -> Uuid,
        item_instance_id -> Nullable<Uuid>,
        item_id -> Nullable<Uuid>,
        inventory_type -> InventoryType,
        slot_index -> Int2,
        quantity -> Int2,
        acquired_at -> Timestamptz,
    }
}

diesel::table! {
    item_instance_gems (id) {
        id -> Uuid,
        item_instance_id -> Uuid,
        slot_index -> Int2,
        gem_instance_id -> Uuid,
        socketed_at -> Timestamptz,
    }
}

diesel::table! {
    item_instances (id) {
        id -> Uuid,
        item_id -> Uuid,
        refinement -> Int2,
        gem_slots -> Int2,
        attributes -> Jsonb,
        owner_character_id -> Nullable<Uuid>,
        owner_account_id -> Nullable<Uuid>,
        in_shared_storage -> Bool,
        in_trade -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ItemRarity;
    use super::sql_types::EquipmentSlotType;
    use super::sql_types::CharacterClass;

    items (id) {
        id -> Uuid,
        #[max_length = 64]
        name -> Varchar,
        description -> Nullable<Text>,
        rarity -> ItemRarity,
        equipment_slot -> Nullable<EquipmentSlotType>,
        class -> Nullable<CharacterClass>,
        level_req -> Int2,
        stats -> Jsonb,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    mob_drop_rates (id) {
        id -> Uuid,
        mob_id -> Uuid,
        item_id -> Uuid,
        drop_chance -> Numeric,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::MobType;

    mobs (id) {
        id -> Uuid,
        #[max_length = 64]
        slug -> Varchar,
        #[max_length = 64]
        name -> Varchar,
        description -> Nullable<Text>,
        mob_type -> MobType,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::CharacterLocation;

    quests (id) {
        id -> Uuid,
        #[max_length = 64]
        name -> Varchar,
        description -> Nullable<Text>,
        city -> Nullable<CharacterLocation>,
        level_req -> Int2,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::CharacterClass;

    skills (id) {
        id -> Uuid,
        #[max_length = 64]
        slug -> Varchar,
        #[max_length = 64]
        name -> Varchar,
        description -> Nullable<Text>,
        character_class -> CharacterClass,
        line_id -> Nullable<Uuid>,
        level_req -> Int2,
        max_level -> Int2,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(character_consumable_slots -> characters (character_id));
diesel::joinable!(character_consumable_slots -> item_instances (item_instance_id));
diesel::joinable!(character_evolution -> characters (character_id));
diesel::joinable!(character_evolution -> evolution_lines (line_id));
diesel::joinable!(character_quests -> characters (character_id));
diesel::joinable!(character_quests -> quests (quest_id));
diesel::joinable!(character_skill_slots -> characters (character_id));
diesel::joinable!(character_skill_slots -> skills (skill_id));
diesel::joinable!(character_skills -> characters (character_id));
diesel::joinable!(character_skills -> skills (skill_id));
diesel::joinable!(characters -> accounts (account_id));
diesel::joinable!(currency_transactions -> accounts (account_id));
diesel::joinable!(currency_transactions -> characters (character_id));
diesel::joinable!(dungeon_history -> characters (character_id));
diesel::joinable!(dungeon_mobs -> mobs (mob_id));
diesel::joinable!(equipment -> characters (character_id));
diesel::joinable!(equipment -> item_instances (item_instance_id));
diesel::joinable!(evolution_steps -> evolution_lines (line_id));
diesel::joinable!(inventory -> characters (character_id));
diesel::joinable!(inventory -> item_instances (item_instance_id));
diesel::joinable!(inventory -> items (item_id));
diesel::joinable!(item_instances -> accounts (owner_account_id));
diesel::joinable!(item_instances -> characters (owner_character_id));
diesel::joinable!(item_instances -> items (item_id));
diesel::joinable!(mob_drop_rates -> items (item_id));
diesel::joinable!(mob_drop_rates -> mobs (mob_id));
diesel::joinable!(skills -> evolution_lines (line_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    character_consumable_slots,
    character_evolution,
    character_quests,
    character_skill_slots,
    character_skills,
    characters,
    currency_transactions,
    dungeon_history,
    dungeon_mobs,
    equipment,
    evolution_lines,
    evolution_steps,
    inventory,
    item_instance_gems,
    item_instances,
    items,
    mob_drop_rates,
    mobs,
    quests,
    skills,
);
