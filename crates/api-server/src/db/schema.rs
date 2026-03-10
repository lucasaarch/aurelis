// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "character_class"))]
    pub struct CharacterClass;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "character_location"))]
    pub struct CharacterLocation;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "item_rarity"))]
    pub struct ItemRarity;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "item_slot"))]
    pub struct ItemSlot;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "quest_status"))]
    pub struct QuestStatus;
}

diesel::table! {
    accounts (id) {
        id -> Uuid,
        #[max_length = 32]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 16]
        region -> Varchar,
        is_banned -> Bool,
        banned_at -> Nullable<Timestamptz>,
        banned_reason -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
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
        hp_current -> Int4,
        hp_max -> Int4,
        mana_current -> Int4,
        mana_max -> Int4,
        location -> CharacterLocation,
        is_online -> Bool,
        last_seen_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
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
    use diesel::sql_types::*;
    use super::sql_types::ItemSlot;

    equipment (character_id, slot) {
        character_id -> Uuid,
        slot -> ItemSlot,
        inventory_id -> Uuid,
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
    inventory (id) {
        id -> Uuid,
        character_id -> Uuid,
        item_id -> Uuid,
        quantity -> Int2,
        acquired_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ItemRarity;
    use super::sql_types::ItemSlot;

    items (id) {
        id -> Uuid,
        #[max_length = 64]
        name -> Varchar,
        description -> Nullable<Text>,
        rarity -> ItemRarity,
        slot -> ItemSlot,
        level_req -> Int2,
        stats -> Jsonb,
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
    refresh_tokens (id) {
        id -> Uuid,
        account_id -> Uuid,
        #[max_length = 255]
        token_hash -> Varchar,
        expires_at -> Timestamptz,
        revoked -> Bool,
        revoked_at -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(character_evolution -> characters (character_id));
diesel::joinable!(character_evolution -> evolution_lines (line_id));
diesel::joinable!(character_quests -> characters (character_id));
diesel::joinable!(character_quests -> quests (quest_id));
diesel::joinable!(characters -> accounts (account_id));
diesel::joinable!(dungeon_history -> characters (character_id));
diesel::joinable!(equipment -> characters (character_id));
diesel::joinable!(equipment -> inventory (inventory_id));
diesel::joinable!(evolution_steps -> evolution_lines (line_id));
diesel::joinable!(inventory -> characters (character_id));
diesel::joinable!(inventory -> items (item_id));
diesel::joinable!(refresh_tokens -> accounts (account_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    character_evolution,
    character_quests,
    characters,
    dungeon_history,
    equipment,
    evolution_lines,
    evolution_steps,
    inventory,
    items,
    quests,
    refresh_tokens,
);
