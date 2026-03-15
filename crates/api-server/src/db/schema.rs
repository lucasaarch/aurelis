// @generated automatically by Diesel CLI.

pub mod sql_types {
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
    character_class_path_classes (id) {
        id -> Uuid,
        #[max_length = 64]
        slug -> Varchar,
        character_class_path_id -> Uuid,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    character_class_paths (id) {
        id -> Uuid,
        character_id -> Uuid,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    characters (id) {
        id -> Uuid,
        #[max_length = 64]
        slug -> Varchar,
        created_at -> Timestamptz,
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
    dungeons (id) {
        id -> Uuid,
        #[max_length = 64]
        slug -> Varchar,
        created_at -> Timestamptz,
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
    use super::sql_types::InventoryType;

    inventory (id) {
        id -> Uuid,
        character_id -> Uuid,
        inventory_type -> InventoryType,
        capacity -> Int2,
        created_at -> Timestamptz,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    inventory_items (id) {
        id -> Uuid,
        inventory_id -> Uuid,
        item_instance_id -> Nullable<Uuid>,
        item_id -> Nullable<Uuid>,
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
        bonus_gem_slots -> Int2,
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
    use super::sql_types::InventoryType;

    items (id) {
        id -> Uuid,
        #[max_length = 64]
        slug -> Varchar,
        inventory_type -> InventoryType,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    mobs (id) {
        id -> Uuid,
        #[max_length = 64]
        slug -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::QuestStatus;

    player_account_quests (id) {
        id -> Uuid,
        account_id -> Uuid,
        quest_id -> Uuid,
        completed_by_character_id -> Nullable<Uuid>,
        status -> QuestStatus,
        progress -> Jsonb,
        #[max_length = 64]
        selected_reward_item_slug -> Nullable<Varchar>,
        started_at -> Nullable<Timestamptz>,
        completed_at -> Nullable<Timestamptz>,
        claimed_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    player_character_dungeon_history (id) {
        id -> Uuid,
        character_id -> Uuid,
        dungeon_id -> Uuid,
        hard_mode -> Bool,
        completed_at -> Timestamptz,
        xp_gained -> Int4,
        duration_secs -> Nullable<Int4>,
    }
}

diesel::table! {
    player_character_mob_kills (id) {
        id -> Uuid,
        character_id -> Uuid,
        mob_id -> Uuid,
        dungeon_id -> Nullable<Uuid>,
        killed_at -> Timestamptz,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::QuestStatus;

    player_character_quests (id) {
        id -> Uuid,
        character_id -> Uuid,
        quest_id -> Uuid,
        status -> QuestStatus,
        progress -> Jsonb,
        #[max_length = 64]
        selected_reward_item_slug -> Nullable<Varchar>,
        started_at -> Nullable<Timestamptz>,
        completed_at -> Nullable<Timestamptz>,
        claimed_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    player_characters (id) {
        id -> Uuid,
        account_id -> Uuid,
        #[max_length = 24]
        name -> Varchar,
        character_id -> Uuid,
        #[max_length = 64]
        current_class_slug -> Varchar,
        level -> Int2,
        experience -> Int8,
        credits -> Int8,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        beginner_skill_unlocked -> Bool,
        intermediate_skill_unlocked -> Bool,
    }
}

diesel::table! {
    quests (id) {
        id -> Uuid,
        #[max_length = 64]
        slug -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(character_class_path_classes -> character_class_paths (character_class_path_id));
diesel::joinable!(character_class_paths -> characters (character_id));
diesel::joinable!(currency_transactions -> accounts (account_id));
diesel::joinable!(currency_transactions -> player_characters (character_id));
diesel::joinable!(equipment -> item_instances (item_instance_id));
diesel::joinable!(equipment -> player_characters (character_id));
diesel::joinable!(inventory -> player_characters (character_id));
diesel::joinable!(inventory_items -> inventory (inventory_id));
diesel::joinable!(inventory_items -> item_instances (item_instance_id));
diesel::joinable!(inventory_items -> items (item_id));
diesel::joinable!(item_instances -> accounts (owner_account_id));
diesel::joinable!(item_instances -> items (item_id));
diesel::joinable!(item_instances -> player_characters (owner_character_id));
diesel::joinable!(player_account_quests -> accounts (account_id));
diesel::joinable!(player_account_quests -> player_characters (completed_by_character_id));
diesel::joinable!(player_account_quests -> quests (quest_id));
diesel::joinable!(player_character_dungeon_history -> dungeons (dungeon_id));
diesel::joinable!(player_character_dungeon_history -> player_characters (character_id));
diesel::joinable!(player_character_mob_kills -> dungeons (dungeon_id));
diesel::joinable!(player_character_mob_kills -> mobs (mob_id));
diesel::joinable!(player_character_mob_kills -> player_characters (character_id));
diesel::joinable!(player_character_quests -> player_characters (character_id));
diesel::joinable!(player_character_quests -> quests (quest_id));
diesel::joinable!(player_characters -> accounts (account_id));
diesel::joinable!(player_characters -> characters (character_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    character_class_path_classes,
    character_class_paths,
    characters,
    currency_transactions,
    dungeons,
    equipment,
    inventory,
    inventory_items,
    item_instance_gems,
    item_instances,
    items,
    mobs,
    player_account_quests,
    player_character_dungeon_history,
    player_character_mob_kills,
    player_character_quests,
    player_characters,
    quests,
);
