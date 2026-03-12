import { cva, type VariantProps } from "class-variance-authority";
import { Slot } from "radix-ui";
import type * as React from "react";

import { cn } from "@/lib/utils";

const buttonVariants = cva(
    "group/button btn-cornered inline-flex shrink-0 items-center justify-center cursor-pointer border border-amber-600/35 bg-clip-padding text-[0.65rem] font-semibold uppercase tracking-[0.22em] text-amber-100/85 transition-all outline-none select-none focus-visible:border-amber-500/70 focus-visible:ring-1 focus-visible:ring-amber-900/30 disabled:pointer-events-none disabled:opacity-50 aria-invalid:border-destructive aria-invalid:ring-3 aria-invalid:ring-destructive/20 dark:aria-invalid:border-destructive/50 dark:aria-invalid:ring-destructive/40 rounded-none [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
    {
        variants: {
            variant: {
                default: [
                    "shadow-[0_10px_24px_rgba(0,0,0,0.45),inset_0_0_30px_rgba(180,140,50,0.05)]",
                    "hover:border-amber-500/70 hover:text-amber-100/95",
                    "hover:shadow-[0_12px_28px_rgba(0,0,0,0.5),0_0_18px_rgba(180,120,30,0.25)]",
                ].join(" "),
                outline:
                    "bg-transparent text-amber-200/80 hover:border-amber-500/70 hover:text-amber-100/90",
                secondary:
                    "border-amber-700/30 bg-[#0b0906] text-amber-200/70 hover:border-amber-500/60 hover:text-amber-100/90",
                ghost: "border-transparent bg-transparent text-amber-200/70 hover:border-amber-600/30 hover:bg-[#0c0a06]/60",
                destructive:
                    "border-red-600/50 text-red-300/80 bg-[#120909] hover:border-red-500/70 hover:text-red-200/90 focus-visible:border-red-500/70 focus-visible:ring-red-900/30",
                link: "bg-transparent text-amber-200/80 underline-offset-4 hover:underline",
                resonaSubmit: [
                    "border border-amber-600/55",
                    "text-amber-100/90",
                    "shadow-[0_12px_30px_rgba(0,0,0,0.55),0_0_22px_rgba(180,120,30,0.2)]",
                    "hover:border-amber-500/80 hover:text-amber-100",
                    "hover:shadow-[0_14px_34px_rgba(0,0,0,0.6),0_0_26px_rgba(180,120,30,0.3)]",
                ].join(" "),
            },
            size: {
                default:
                    "h-8 gap-1.5 px-3 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2",
                xs: "h-6 gap-1 px-2 text-[0.6rem] has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&_svg:not([class*='size-'])]:size-3",
                sm: "h-7 gap-1 px-2.5 text-[0.65rem] has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&_svg:not([class*='size-'])]:size-3.5",
                lg: "h-9 gap-1.5 px-3.5 text-[0.7rem] has-data-[icon=inline-end]:pr-3 has-data-[icon=inline-start]:pl-3",
                icon: "size-8",
                "icon-xs": "size-6 [&_svg:not([class*='size-'])]:size-3",
                "icon-sm": "size-7",
                "icon-lg": "size-9",
            },
        },
        defaultVariants: {
            variant: "default",
            size: "default",
        },
    },
);

function Button({
    className,
    variant = "default",
    size = "default",
    asChild = false,
    ...props
}: React.ComponentProps<"button"> &
    VariantProps<typeof buttonVariants> & {
        asChild?: boolean;
    }) {
    const Comp = asChild ? Slot.Root : "button";

    return (
        <Comp
            data-slot="button"
            data-variant={variant}
            data-size={size}
            className={cn(buttonVariants({ variant, size, className }))}
            {...props}
        />
    );
}

export { Button, buttonVariants };
