ALTER TYPE equipment_slot_type RENAME TO equipment_slot_type_old;

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

ALTER
TABLE equipment
    ALTER COLUMN slot TYPE equipment_slot_type
    USING slot::text::equipment_slot_type;

ALTER TABLE items
    ALTER COLUMN equipment_slot TYPE equipment_slot_type
    USING equipment_slot::text::equipment_slot_type;

DROP TYPE equipment_slot_type_old;