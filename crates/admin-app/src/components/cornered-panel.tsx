import type * as React from "react";

import { cn } from "@/lib/utils";

export function CorneredPanel({
    children,
    className,
    ...props
}: React.ComponentProps<"div">) {
    return (
        <div
            className={cn(
                "relative rounded-none border border-amber-600/20 bg-linear-to-b from-[#12100a]/95 to-[#0c0a06]/98 p-8",
                "shadow-[0_20px_60px_rgba(0,0,0,0.6),inset_0_0_80px_rgba(180,140,50,0.02)]",
                className,
            )}
            {...props}
        >
            <div className="absolute left-[10%] right-[10%] top-0 h-px bg-linear-to-r from-transparent via-amber-500/50 to-transparent" />
            <div className="absolute bottom-0 left-[20%] right-[20%] h-px bg-linear-to-r from-transparent via-amber-600/20 to-transparent" />

            <div className="absolute left-0 top-0 h-3 w-3 border-l border-t border-amber-500/40" />
            <div className="absolute right-0 top-0 h-3 w-3 border-r border-t border-amber-500/40" />
            <div className="absolute bottom-0 left-0 h-3 w-3 border-b border-l border-amber-500/40" />
            <div className="absolute bottom-0 right-0 h-3 w-3 border-b border-r border-amber-500/40" />

            {children}
        </div>
    );
}
