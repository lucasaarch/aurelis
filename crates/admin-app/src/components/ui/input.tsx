import type * as React from "react";
import { cn } from "@/lib/utils";

function Input({ className, type, ...props }: React.ComponentProps<"input">) {
    return (
        <input
            type={type}
            data-slot="input"
            className={cn(
                "h-8 w-full min-w-0 rounded-sm border border-amber-600/20 bg-[#0a0805] px-2.5 py-1 text-sm text-amber-100/90 transition-colors outline-none",
                "placeholder:text-amber-600/60",
                "focus-visible:border-amber-600/40 focus-visible:ring-1 focus-visible:ring-amber-900/30",
                "disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-40",
                "aria-invalid:border-red-800/60 aria-invalid:ring-1 aria-invalid:ring-red-900/30",
                "file:inline-flex file:h-6 file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-amber-200/60",
                "[appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none",
                className,
            )}
            {...props}
        />
    );
}

export { Input };
