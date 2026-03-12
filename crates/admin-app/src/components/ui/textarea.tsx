import * as React from "react";

import { cn } from "@/lib/utils";

const Textarea = React.forwardRef<
    HTMLTextAreaElement,
    React.ComponentProps<"textarea">
>(({ className, ...props }, ref) => {
    return (
        <textarea
            ref={ref}
            data-slot="textarea"
            className={cn(
                "field-cornered flex min-h-16 w-full rounded-none border border-amber-600/25 px-2.5 py-2 text-sm text-amber-100/90 transition-colors outline-none resize-none",
                "placeholder:text-amber-600/60",
                "focus-visible:border-amber-600/40 focus-visible:ring-1 focus-visible:ring-amber-900/30",
                "disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-40",
                "aria-invalid:border-red-800/60 aria-invalid:ring-1 aria-invalid:ring-red-900/30",
                className,
            )}
            {...props}
        />
    );
});

Textarea.displayName = "Textarea";

export { Textarea };
