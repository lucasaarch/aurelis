-- Your SQL goes here
ALTER TABLE items
    ADD COLUMN inventory_type inventory_type NOT NULL;

CREATE INDEX idx_items_inventory_type ON items (inventory_type);