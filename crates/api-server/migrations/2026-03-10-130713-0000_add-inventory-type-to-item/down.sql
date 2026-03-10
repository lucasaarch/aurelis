-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS idx_items_inventory_type;

ALTER TABLE items
    DROP COLUMN IF EXISTS inventory_type;