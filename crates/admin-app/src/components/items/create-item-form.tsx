import { useMutation } from "@tanstack/react-query";
import { CheckCircle, Loader2 } from "lucide-react";
import { Controller, useForm } from "react-hook-form";

import { Button } from "@/components/ui/button";
import { FieldItem } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select";
import { Textarea } from "@/components/ui/textarea";
import type { CreateItemRequest } from "@/lib/api";
import { AdminService } from "@/lib/api";
import { displayEnum } from "@/lib/utils";

type CreateItemValues = {
    name: string;
    rarity: string;
    inventoryType: string;
    class: string;
    equipmentSlot: string;
    levelReq: string;
    maxStack: string;
    description: string;
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

export function CreateItemForm({ onSuccess }: { onSuccess: () => void }) {
    const { register, handleSubmit, reset, control } =
        useForm<CreateItemValues>({
            defaultValues: {
                name: "",
                rarity: "common",
                inventoryType: "equipment",
                class: "_none",
                equipmentSlot: "_none",
                levelReq: "",
                maxStack: "",
                description: "",
            },
        });

    const mutation = useMutation({
        mutationFn: (values: CreateItemValues) => {
            const body: CreateItemRequest = {
                name: values.name,
                rarity: values.rarity,
                inventoryType: values.inventoryType,
                class: values.class === "_none" ? null : values.class,
                equipmentSlot:
                    values.equipmentSlot === "_none"
                        ? null
                        : values.equipmentSlot,
                levelReq: values.levelReq ? Number(values.levelReq) : null,
                maxStack: values.maxStack ? Number(values.maxStack) : null,
                description: values.description || null,
            };
            return AdminService.createItem({ requestBody: body });
        },
        onSuccess: () => {
            reset();
            onSuccess();
        },
    });

    return (
        <form
            onSubmit={handleSubmit((v) => mutation.mutate(v))}
            className="space-y-4"
        >
            <div className="grid grid-cols-2 gap-3 sm:grid-cols-3">
                <FieldItem label="Name">
                    <Input
                        {...register("name", { required: true })}
                        placeholder="Void Sword"
                    />
                </FieldItem>

                <FieldItem label="Rarity">
                    <Controller
                        name="rarity"
                        control={control}
                        render={({ field }) => (
                            <Select
                                value={field.value}
                                onValueChange={field.onChange}
                            >
                                <SelectTrigger>
                                    <SelectValue />
                                </SelectTrigger>
                                <SelectContent position="popper">
                                    {itemRarities.map((rarity) => (
                                        <SelectItem key={rarity} value={rarity}>
                                            {displayEnum(rarity)}
                                        </SelectItem>
                                    ))}
                                </SelectContent>
                            </Select>
                        )}
                    />
                </FieldItem>

                <FieldItem label="Inventory type">
                    <Controller
                        name="inventoryType"
                        control={control}
                        render={({ field }) => (
                            <Select
                                value={field.value}
                                onValueChange={field.onChange}
                            >
                                <SelectTrigger>
                                    <SelectValue />
                                </SelectTrigger>
                                <SelectContent position="popper">
                                    {inventoryTypes.map((type) => (
                                        <SelectItem key={type} value={type}>
                                            {displayEnum(type)}
                                        </SelectItem>
                                    ))}
                                </SelectContent>
                            </Select>
                        )}
                    />
                </FieldItem>

                <FieldItem label="Class (optional)">
                    <Controller
                        name="class"
                        control={control}
                        render={({ field }) => (
                            <Select
                                value={field.value}
                                onValueChange={field.onChange}
                            >
                                <SelectTrigger>
                                    <SelectValue placeholder="— All —" />
                                </SelectTrigger>
                                <SelectContent position="popper">
                                    <SelectItem value="_none">
                                        — All —
                                    </SelectItem>
                                    {characterClasses.map((cls) => (
                                        <SelectItem key={cls} value={cls}>
                                            {displayEnum(cls)}
                                        </SelectItem>
                                    ))}
                                </SelectContent>
                            </Select>
                        )}
                    />
                </FieldItem>

                <FieldItem label="Equipment slot (optional)">
                    <Controller
                        name="equipmentSlot"
                        control={control}
                        render={({ field }) => (
                            <Select
                                value={field.value}
                                onValueChange={field.onChange}
                            >
                                <SelectTrigger>
                                    <SelectValue placeholder="— None —" />
                                </SelectTrigger>
                                <SelectContent position="popper">
                                    <SelectItem value="_none">
                                        — None —
                                    </SelectItem>
                                    {equipmentSlots.map((slot) => (
                                        <SelectItem key={slot} value={slot}>
                                            {displayEnum(slot)}
                                        </SelectItem>
                                    ))}
                                </SelectContent>
                            </Select>
                        )}
                    />
                </FieldItem>

                <FieldItem label="Level requirement (optional)">
                    <Input
                        {...register("levelReq")}
                        type="number"
                        min={1}
                        max={40}
                        placeholder="1–40"
                    />
                </FieldItem>

                <FieldItem label="Max stack (optional)">
                    <Input
                        {...register("maxStack")}
                        type="number"
                        min={1}
                        placeholder="1+"
                    />
                </FieldItem>
            </div>

            <FieldItem label="Description (optional)">
                <Textarea
                    {...register("description")}
                    placeholder="Item description…"
                    rows={3}
                />
            </FieldItem>

            <div className="flex items-center gap-3 pt-1">
                <Button
                    type="submit"
                    disabled={mutation.isPending}
                    variant="resonaSubmit"
                >
                    <span className="flex items-center gap-2 px-1">
                        {mutation.isPending && (
                            <Loader2 className="size-3.5 animate-spin" />
                        )}
                        {mutation.isPending ? "Creating…" : "Create item"}
                    </span>
                </Button>

                {mutation.isSuccess && (
                    <span className="flex items-center gap-1.5 text-[0.7rem] text-emerald-500/70">
                        <CheckCircle className="size-3.5" />
                        Item created:{" "}
                        <span className="text-emerald-400/80">
                            {mutation.data.name}
                        </span>
                        <span className="text-amber-800/40">
                            ({mutation.data.slug})
                        </span>
                    </span>
                )}

                {mutation.isError && (
                    <span className="text-[0.7rem] text-red-500/70">
                        Failed to create item.
                    </span>
                )}
            </div>
        </form>
    );
}
