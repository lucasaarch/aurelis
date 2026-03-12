"use client";

import { Controller, useWatch, type Control, type FieldValues, type Path } from "react-hook-form";

import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select";

export type FormSelectOption = {
    value: string;
    label: string;
};

export function FormSelect<TFieldValues extends FieldValues>({
    control,
    name,
    options,
    placeholder = "Select",
    contentPosition = "popper",
    size = "default",
}: {
    control: Control<TFieldValues>;
    name: Path<TFieldValues>;
    options: FormSelectOption[];
    placeholder?: string;
    contentPosition?: "popper" | "item-aligned";
    size?: "sm" | "default";
}) {
    const watchedValue = useWatch({ control, name });

    return (
        <Controller
            name={name}
            control={control}
            render={({ field }) => (
                <Select
                    key={`form-select-${name}-${watchedValue ?? "empty"}`}
                    value={(watchedValue ?? field.value ?? "") as string}
                    onValueChange={field.onChange}
                >
                    <SelectTrigger size={size}>
                        <SelectValue placeholder={placeholder} />
                    </SelectTrigger>
                    <SelectContent position={contentPosition}>
                        {options.map((option) => (
                            <SelectItem key={option.value} value={option.value}>
                                {option.label}
                            </SelectItem>
                        ))}
                    </SelectContent>
                </Select>
            )}
        />
    );
}
