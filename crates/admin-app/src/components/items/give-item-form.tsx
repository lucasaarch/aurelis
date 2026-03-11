import { useMutation } from "@tanstack/react-query";
import { CheckCircle, Loader2 } from "lucide-react";
import { useForm } from "react-hook-form";

import { Button } from "@/components/ui/button";
import { FieldItem } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import type { GiveItemRequest } from "@/lib/api";
import { AdminService } from "@/lib/api";

type GiveItemValues = {
    characterId: string;
    itemId: string;
    quantity: string;
};

export function GiveItemForm() {
    const { register, handleSubmit, reset } = useForm<GiveItemValues>({
        defaultValues: { characterId: "", itemId: "", quantity: "" },
    });

    const mutation = useMutation({
        mutationFn: (values: GiveItemValues) => {
            const body: GiveItemRequest = {
                characterId: values.characterId,
                itemId: values.itemId,
                quantity: values.quantity ? Number(values.quantity) : null,
            };
            return AdminService.giveItem({ requestBody: body });
        },
        onSuccess: () => reset(),
    });

    return (
        <form
            onSubmit={handleSubmit((v) => mutation.mutate(v))}
            className="space-y-4"
        >
            <div className="grid grid-cols-1 gap-3 sm:grid-cols-3">
                <FieldItem label="Character ID">
                    <Input
                        {...register("characterId", { required: true })}
                        placeholder="uuid"
                        className="font-mono text-xs"
                    />
                </FieldItem>
                <FieldItem label="Item ID">
                    <Input
                        {...register("itemId", { required: true })}
                        placeholder="uuid"
                        className="font-mono text-xs"
                    />
                </FieldItem>
                <FieldItem label="Quantity (optional)">
                    <Input
                        {...register("quantity")}
                        type="number"
                        min={1}
                        placeholder="1"
                    />
                </FieldItem>
            </div>

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
                        {mutation.isPending ? "Sending…" : "Give item"}
                    </span>
                </Button>

                {mutation.isSuccess && (
                    <span className="flex items-center gap-1.5 text-[0.7rem] text-emerald-500/70">
                        <CheckCircle className="size-3.5" />
                        Item delivered successfully.
                    </span>
                )}

                {mutation.isError && (
                    <span className="text-[0.7rem] text-red-500/70">
                        Failed to give item.
                    </span>
                )}
            </div>
        </form>
    );
}
