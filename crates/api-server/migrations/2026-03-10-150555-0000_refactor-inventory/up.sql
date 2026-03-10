CREATE TABLE
    inventory_new (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        character_id UUID NOT NULL REFERENCES characters (id) ON DELETE CASCADE,
        inventory_type inventory_type NOT NULL,
        capacity SMALLINT NOT NULL DEFAULT 56,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW (),
        UNIQUE (character_id, inventory_type)
    );

CREATE TABLE
    inventory_items (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        inventory_id UUID NOT NULL REFERENCES inventory_new (id) ON DELETE CASCADE,
        item_instance_id UUID REFERENCES item_instances (id) ON DELETE SET NULL,
        item_id UUID REFERENCES items (id),
        slot_index SMALLINT NOT NULL CHECK (slot_index >= 0),
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

INSERT INTO inventory_new (character_id, inventory_type, capacity)
SELECT DISTINCT
    i.character_id,
    i.inventory_type,
    CASE i.inventory_type
        WHEN 'equipment'  THEN c.equipment_inventory_capacity
        WHEN 'accessory'  THEN c.accessory_inventory_capacity
        WHEN 'consumable' THEN c.consumable_inventory_capacity
        WHEN 'material'   THEN c.material_inventory_capacity
        WHEN 'quest_item' THEN c.quest_item_inventory_capacity
        WHEN 'special'    THEN c.special_inventory_capacity
    END
FROM inventory i
JOIN characters c ON c.id = i.character_id;

INSERT INTO inventory_items (inventory_id, item_instance_id, item_id, slot_index, quantity, acquired_at)
SELECT
    inv_new.id,
    i.item_instance_id,
    i.item_id,
    i.slot_index,
    i.quantity,
    i.acquired_at
FROM inventory i
JOIN inventory_new inv_new ON inv_new.character_id = i.character_id
    AND inv_new.inventory_type = i.inventory_type;

INSERT INTO inventory_new (character_id, inventory_type, capacity)
SELECT
    c.id,
    t.inventory_type,
    56
FROM characters c
CROSS JOIN (
    SELECT unnest(ARRAY[
        'equipment'::inventory_type,
        'accessory'::inventory_type,
        'consumable'::inventory_type,
        'material'::inventory_type,
        'quest_item'::inventory_type,
        'special'::inventory_type
    ]) AS inventory_type
) t
ON CONFLICT (character_id, inventory_type) DO NOTHING;

DROP TABLE inventory;

ALTER TABLE inventory_new RENAME TO inventory;

ALTER TABLE characters
    DROP COLUMN equipment_inventory_capacity,
    DROP COLUMN accessory_inventory_capacity,
    DROP COLUMN consumable_inventory_capacity,
    DROP COLUMN material_inventory_capacity,
    DROP COLUMN quest_item_inventory_capacity,
    DROP COLUMN special_inventory_capacity;

CREATE INDEX idx_inventory_character_id    ON inventory (character_id);
CREATE INDEX idx_inventory_items_inventory ON inventory_items (inventory_id);
CREATE INDEX idx_inventory_items_instance  ON inventory_items (item_instance_id);
CREATE INDEX idx_inventory_items_item      ON inventory_items (item_id);