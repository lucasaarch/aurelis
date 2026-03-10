-- This file should undo anything in `up.sql`
ALTER TYPE equipment_slot_type RENAME TO equipment_slot_type_old;

CREATE TYPE equipment_slot_type AS ENUM ('weapon', 'head', 'chest', 'legs', 'accessory');

ALTER TABLE equipment
    ALTER COLUMN slot TYPE equipment_slot_type
    USING slot::text::equipment_slot_type;

ALTER TABLE items
    ALTER COLUMN equipment_slot TYPE equipment_slot_type
    USING equipment_slot::text::equipment_slot_type;

DROP TYPE equipment_slot_type_old;