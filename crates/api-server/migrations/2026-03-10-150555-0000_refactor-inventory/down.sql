ALTER TABLE characters
    ADD COLUMN equipment_inventory_capacity  SMALLINT NOT NULL DEFAULT 56,
    ADD COLUMN accessory_inventory_capacity  SMALLINT NOT NULL DEFAULT 56,
    ADD COLUMN consumable_inventory_capacity SMALLINT NOT NULL DEFAULT 56,
    ADD COLUMN material_inventory_capacity   SMALLINT NOT NULL DEFAULT 56,
    ADD COLUMN quest_item_inventory_capacity SMALLINT NOT NULL DEFAULT 56,
    ADD COLUMN special_inventory_capacity    SMALLINT NOT NULL DEFAULT 56;

UPDATE characters c
SET
    equipment_inventory_capacity  = (SELECT capacity FROM inventory WHERE character_id = c.id AND inventory_type = 'equipment'  LIMIT 1),
    accessory_inventory_capacity  = (SELECT capacity FROM inventory WHERE character_id = c.id AND inventory_type = 'accessory'  LIMIT 1),
    consumable_inventory_capacity = (SELECT capacity FROM inventory WHERE character_id = c.id AND inventory_type = 'consumable' LIMIT 1),
    material_inventory_capacity   = (SELECT capacity FROM inventory WHERE character_id = c.id AND inventory_type = 'material'   LIMIT 1),
    quest_item_inventory_capacity = (SELECT capacity FROM inventory WHERE character_id = c.id AND inventory_type = 'quest_item' LIMIT 1),
    special_inventory_capacity    = (SELECT capacity FROM inventory WHERE character_id = c.id AND inventory_type = 'special'    LIMIT 1);

CREATE TABLE inventory_old (
    id               UUID           PRIMARY KEY DEFAULT uuid_generate_v4(),
    character_id     UUID           NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    item_instance_id UUID           REFERENCES item_instances(id) ON DELETE SET NULL,
    item_id          UUID           REFERENCES items(id),
    inventory_type   inventory_type NOT NULL,
    slot_index       SMALLINT       NOT NULL CHECK (slot_index >= 0),
    quantity         SMALLINT       NOT NULL DEFAULT 1,
    acquired_at      TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    UNIQUE (character_id, inventory_type, slot_index),
    CONSTRAINT chk_inventory_item CHECK (
        (item_instance_id IS NOT NULL AND item_id IS NULL) OR
        (item_instance_id IS NULL AND item_id IS NOT NULL)
    )
);

INSERT INTO inventory_old (character_id, item_instance_id, item_id, inventory_type, slot_index, quantity, acquired_at)
SELECT
    inv.character_id,
    ii.item_instance_id,
    ii.item_id,
    inv.inventory_type,
    ii.slot_index,
    ii.quantity,
    ii.acquired_at
FROM inventory_items ii
JOIN inventory inv ON inv.id = ii.inventory_id;

DROP INDEX IF EXISTS idx_inventory_items_item;
DROP INDEX IF EXISTS idx_inventory_items_instance;
DROP INDEX IF EXISTS idx_inventory_items_inventory;
DROP INDEX IF EXISTS idx_inventory_character_id;

DROP TABLE inventory_items;
DROP TABLE inventory;

ALTER TABLE inventory_old RENAME TO inventory;

CREATE INDEX idx_inventory_character_id ON inventory (character_id);