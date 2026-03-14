CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE currency_origin AS ENUM (
    'purchase',
    'trade',
    'bonus',
    'dungeon',
    'quest',
    'npc'
);

CREATE TYPE equipment_slot_type AS ENUM (
    'weapon',
    'head',
    'chest',
    'legs',
    'gloves',
    'shoes',
    'acc_ring_1',
    'acc_ring_2',
    'acc_necklace',
    'acc_earrings',
    'acc_arm',
    'acc_face_bottom',
    'acc_face_middle',
    'acc_face_top',
    'acc_bottom_piece',
    'acc_top_piece',
    'acc_weapon',
    'acc_support_unit'
);

CREATE TYPE inventory_type AS ENUM (
    'equipment',
    'accessory',
    'consumable',
    'material',
    'quest_item',
    'special'
);

CREATE TYPE quest_status AS ENUM ('available', 'in_progress', 'completed');

CREATE TABLE
    accounts (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        email VARCHAR(255) NOT NULL UNIQUE,
        password_hash VARCHAR(255) NOT NULL,
        max_characters SMALLINT NOT NULL DEFAULT 3,
        shared_storage_enabled BOOLEAN NOT NULL DEFAULT FALSE,
        shared_storage_capacity SMALLINT NOT NULL DEFAULT 20,
        cash BIGINT NOT NULL DEFAULT 0,
        stored_credits BIGINT NOT NULL DEFAULT 0,
        is_admin BOOLEAN NOT NULL DEFAULT FALSE,
        god_mode BOOLEAN NOT NULL DEFAULT FALSE,
        email_verified BOOLEAN NOT NULL DEFAULT FALSE,
        email_verified_at TIMESTAMPTZ,
        email_verify_token VARCHAR(255),
        email_verify_token_expires TIMESTAMPTZ,
        password_reset_token VARCHAR(255),
        password_reset_expires TIMESTAMPTZ,
        banned_at TIMESTAMPTZ,
        banned_reason TEXT,
        suspended_until TIMESTAMPTZ,
        chat_restricted_until TIMESTAMPTZ,
        last_login_at TIMESTAMPTZ,
        last_login_ip VARCHAR(45),
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        deleted_at TIMESTAMPTZ
    );

CREATE TABLE
    characters (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        slug VARCHAR(64) NOT NULL UNIQUE,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );

CREATE TABLE
    player_characters (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        account_id UUID NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
        name VARCHAR(24) NOT NULL UNIQUE,
        character_id UUID NOT NULL REFERENCES characters (id),
        current_class_slug VARCHAR(64) NOT NULL,
        level SMALLINT NOT NULL DEFAULT 1,
        experience BIGINT NOT NULL DEFAULT 0,
        credits BIGINT NOT NULL DEFAULT 0,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );

CREATE TABLE
    items (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        slug VARCHAR(64) NOT NULL UNIQUE,
        inventory_type inventory_type NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );

CREATE TABLE
    dungeons (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        slug VARCHAR(64) NOT NULL UNIQUE,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );

CREATE TABLE
    mobs (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        slug VARCHAR(64) NOT NULL UNIQUE,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );

CREATE TABLE
    item_instances (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        item_id UUID NOT NULL REFERENCES items (id),
        refinement SMALLINT NOT NULL DEFAULT 0,
        gem_slots SMALLINT NOT NULL DEFAULT 0,
        attributes JSONB NOT NULL DEFAULT '{}',
        owner_character_id UUID REFERENCES player_characters (id) ON DELETE SET NULL,
        owner_account_id UUID REFERENCES accounts (id) ON DELETE SET NULL,
        in_shared_storage BOOLEAN NOT NULL DEFAULT FALSE,
        in_trade BOOLEAN NOT NULL DEFAULT FALSE,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );

CREATE TABLE
    item_instance_gems (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        item_instance_id UUID NOT NULL REFERENCES item_instances (id) ON DELETE CASCADE,
        slot_index SMALLINT NOT NULL CHECK (slot_index >= 0),
        gem_instance_id UUID NOT NULL REFERENCES item_instances (id),
        socketed_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        UNIQUE (item_instance_id, slot_index)
    );

CREATE TABLE
    inventory (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        character_id UUID NOT NULL REFERENCES player_characters (id) ON DELETE CASCADE,
        inventory_type inventory_type NOT NULL,
        capacity SMALLINT NOT NULL DEFAULT 56,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        UNIQUE (character_id, inventory_type)
    );

CREATE TABLE
    inventory_items (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        inventory_id UUID NOT NULL REFERENCES inventory (id) ON DELETE CASCADE,
        item_instance_id UUID REFERENCES item_instances (id) ON DELETE SET NULL,
        item_id UUID REFERENCES items (id),
        slot_index SMALLINT NOT NULL,
        quantity SMALLINT NOT NULL DEFAULT 1,
        acquired_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        UNIQUE (inventory_id, slot_index),
        CONSTRAINT chk_inventory_item CHECK (
            (
                item_instance_id IS NOT NULL
                AND item_id IS NULL
            )
            OR (
                item_instance_id IS NULL
                AND item_id IS NOT NULL
            )
        )
    );

CREATE TABLE
    equipment (
        character_id UUID NOT NULL REFERENCES player_characters (id) ON DELETE CASCADE,
        slot equipment_slot_type NOT NULL,
        item_instance_id UUID NOT NULL REFERENCES item_instances (id),
        equipped_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        PRIMARY KEY (character_id, slot)
    );

CREATE TABLE
    quests (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        slug VARCHAR(64) NOT NULL UNIQUE,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );

CREATE TABLE
    player_character_quests (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        character_id UUID NOT NULL REFERENCES player_characters (id) ON DELETE CASCADE,
        quest_id UUID NOT NULL REFERENCES quests (id),
        status quest_status NOT NULL DEFAULT 'available',
        progress JSONB NOT NULL DEFAULT '{}',
        selected_reward_item_slug VARCHAR(64),
        started_at TIMESTAMPTZ,
        completed_at TIMESTAMPTZ,
        claimed_at TIMESTAMPTZ,
        UNIQUE (character_id, quest_id)
    );

CREATE TABLE
    player_account_quests (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        account_id UUID NOT NULL REFERENCES accounts (id) ON DELETE CASCADE,
        quest_id UUID NOT NULL REFERENCES quests (id),
        completed_by_character_id UUID REFERENCES player_characters (id) ON DELETE SET NULL,
        status quest_status NOT NULL DEFAULT 'available',
        progress JSONB NOT NULL DEFAULT '{}',
        selected_reward_item_slug VARCHAR(64),
        started_at TIMESTAMPTZ,
        completed_at TIMESTAMPTZ,
        claimed_at TIMESTAMPTZ,
        UNIQUE (account_id, quest_id)
    );

CREATE TABLE
    player_character_dungeon_history (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        character_id UUID NOT NULL REFERENCES player_characters (id) ON DELETE CASCADE,
        dungeon_id UUID NOT NULL REFERENCES dungeons (id),
        hard_mode BOOLEAN NOT NULL DEFAULT FALSE,
        completed_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        xp_gained INT NOT NULL DEFAULT 0,
        duration_secs INT
    );

CREATE TABLE
    player_character_mob_kills (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        character_id UUID NOT NULL REFERENCES player_characters (id) ON DELETE CASCADE,
        mob_id UUID NOT NULL REFERENCES mobs (id),
        dungeon_id UUID REFERENCES dungeons (id),
        killed_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );

CREATE TABLE
    character_class_paths (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        character_id UUID NOT NULL REFERENCES characters (id) ON DELETE CASCADE,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );

CREATE TABLE
    character_class_path_classes (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        slug VARCHAR(64) NOT NULL UNIQUE,
        character_class_path_id UUID NOT NULL REFERENCES character_class_paths (id) ON DELETE CASCADE,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        UNIQUE (character_class_path_id, slug)
    );

CREATE TABLE
    currency_transactions (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        account_id UUID REFERENCES accounts (id) ON DELETE SET NULL,
        character_id UUID REFERENCES player_characters (id) ON DELETE SET NULL,
        currency VARCHAR(16) NOT NULL,
        amount BIGINT NOT NULL,
        balance_after BIGINT NOT NULL,
        origin currency_origin NOT NULL,
        reference_id UUID,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW ()
    );


CREATE INDEX idx_player_characters_account_id ON player_characters (account_id);

CREATE INDEX idx_inventory_character_id ON inventory (character_id);

CREATE INDEX idx_inventory_items_inventory ON inventory_items (inventory_id);

CREATE INDEX idx_inventory_items_instance ON inventory_items (item_instance_id);

CREATE INDEX idx_inventory_items_item ON inventory_items (item_id);

CREATE INDEX idx_player_character_quests_char ON player_character_quests (character_id);

CREATE INDEX idx_player_account_quests_account ON player_account_quests (account_id);

CREATE INDEX idx_player_character_dungeon_history_char ON player_character_dungeon_history (character_id);

CREATE INDEX idx_player_character_mob_kills_character ON player_character_mob_kills (character_id);

CREATE INDEX idx_player_character_mob_kills_mob_id ON player_character_mob_kills (mob_id);

CREATE INDEX idx_player_character_mob_kills_dungeon_id ON player_character_mob_kills (dungeon_id);

CREATE INDEX idx_player_character_mob_kills_character_mob ON player_character_mob_kills (character_id, mob_id);

CREATE INDEX idx_character_class_paths_character_id ON character_class_paths (character_id);

CREATE INDEX idx_character_class_path_classes_path_id ON character_class_path_classes (character_class_path_id);

CREATE INDEX idx_currency_transactions_acc ON currency_transactions (account_id);

CREATE INDEX idx_currency_transactions_char ON currency_transactions (character_id);

CREATE INDEX idx_item_instances_character ON item_instances (owner_character_id);

CREATE INDEX idx_item_instances_account ON item_instances (owner_account_id);

CREATE INDEX idx_items_slug ON items (slug);

CREATE INDEX idx_items_inventory_type ON items (inventory_type);

CREATE INDEX idx_dungeons_slug ON dungeons (slug);

CREATE INDEX idx_mobs_slug ON mobs (slug);
