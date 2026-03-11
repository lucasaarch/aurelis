import { Label as LabelPrimitive } from "radix-ui";
import type * as React from "react";
import { cva, type VariantProps } from "class-variance-authority";

import { cn } from "@/lib/utils";

const labelVariants = cva(
    "flex items-center gap-2 text-sm leading-none font-medium select-none group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50 peer-disabled:cursor-not-allowed peer-disabled:opacity-50",
    {
        variants: {
            variant: {
                default: "",
                caps: "text-[0.65rem] font-semibold uppercase tracking-[0.18em] text-amber-600/80",
            },
        },
        defaultVariants: {
            variant: "default",
        },
    },
);

function Label({
    className,
    variant,
    ...props
}: React.ComponentProps<typeof LabelPrimitive.Root> &
    VariantProps<typeof labelVariants>) {
    return (
        <LabelPrimitive.Root
            data-slot="label"
            data-variant={variant}
            className={cn(labelVariants({ variant }), className)}
            {...props}
        />
    );
}

export { Label };
