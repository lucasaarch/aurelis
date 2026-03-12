"use client";

import type { UseFormReturn } from "react-hook-form";

import { Button } from "@/components/ui/button";
import { FieldItem } from "@/components/ui/field";
import { FormSelect } from "@/components/ui/form-select";
import { Input } from "@/components/ui/input";
import { displayEnum } from "@/lib/utils";

export type ItemSearchValues = {
    search: string;
    rarity: string;
    inventoryType: string;
    class: string;
    equipmentSlot: string;
    levelMin: string;
    levelMax: string;
};

const itemRarities = ["common", "uncommon", "rare", "epic"] as const;
const inventoryTypes = [
    "equipment",
    "accessory",
    "consumable",
    "material",
    "quest_item",
    "special",
] as const;
const characterClasses = ["kael", "rin", "sirena"] as const;
const equipmentSlots = [
    "weapon",
    "head",
    "chest",
    "legs",
    "gloves",
    "shoes",
    "acc_ring_1",
    "acc_ring_2",
    "acc_necklace",
    "acc_earrings",
    "acc_arm",
    "acc_face_bottom",
    "acc_face_middle",
    "acc_face_top",
    "acc_bottom_piece",
    "acc_top_piece",
    "acc_weapon",
    "acc_support_unit",
] as const;

const rarityOptions = [
    { value: "_all", label: "All" },
    ...itemRarities.map((rarity) => ({
        value: rarity,
        label: displayEnum(rarity),
    })),
];

const inventoryTypeOptions = [
    { value: "_all", label: "All" },
    ...inventoryTypes.map((type) => ({
        value: type,
        label: displayEnum(type),
    })),
];

const classOptions = [
    { value: "_all", label: "All" },
    ...characterClasses.map((cls) => ({
        value: cls,
        label: displayEnum(cls),
    })),
];

const equipmentSlotOptions = [
    { value: "_all", label: "All" },
    ...equipmentSlots.map((slot) => ({
        value: slot,
        label: displayEnum(slot),
    })),
];

export function ItemSearchForm({
    form,
    onSubmit,
}: {
    form: UseFormReturn<ItemSearchValues>;
    onSubmit: (values: ItemSearchValues) => void;
}) {
    return (
        <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-4 pb-2">
            <div className="grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
                <FieldItem label="Search">
                    <Input
                        {...form.register("search")}
                        placeholder="Search by name"
                    />
                </FieldItem>

                <FieldItem label="Rarity">
                    <FormSelect
                        control={form.control}
                        name="rarity"
                        options={rarityOptions}
                        placeholder="All"
                    />
                </FieldItem>

                <FieldItem label="Inventory type">
                    <FormSelect
                        control={form.control}
                        name="inventoryType"
                        options={inventoryTypeOptions}
                        placeholder="All"
                    />
                </FieldItem>

                <FieldItem label="Class">
                    <FormSelect
                        control={form.control}
                        name="class"
                        options={classOptions}
                        placeholder="All"
                    />
                </FieldItem>

                <FieldItem label="Equipment slot">
                    <FormSelect
                        control={form.control}
                        name="equipmentSlot"
                        options={equipmentSlotOptions}
                        placeholder="All"
                    />
                </FieldItem>

                <FieldItem label="Level min">
                    <Input
                        {...form.register("levelMin")}
                        type="number"
                        min={1}
                        max={40}
                        placeholder="1"
                    />
                </FieldItem>

                <FieldItem label="Level max">
                    <Input
                        {...form.register("levelMax")}
                        type="number"
                        min={1}
                        max={40}
                        placeholder="40"
                    />
                </FieldItem>
            </div>

            <div className="flex items-center gap-3">
                <Button type="submit" variant="resonaSubmit">
                    Apply filters
                </Button>
            </div>
        </form>
    );
}
