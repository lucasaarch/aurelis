import type * as React from "react";

import { cn } from "@/lib/utils";

function SectionCard({
    title,
    children,
    className,
    titleClassName,
    ...props
}: React.ComponentProps<"section"> & {
    title?: string;
    titleClassName?: string;
}) {
    return (
        <section
            data-slot="section-card"
            className={cn(
                "relative rounded-none border border-amber-600/10 bg-[#0c0a06]/60 p-5",
                className,
            )}
            {...props}
        >
            <div className="absolute left-0 top-0 h-3 w-3 border-l border-t border-amber-600/20" />
            <div className="absolute right-0 top-0 h-3 w-3 border-r border-t border-amber-600/20" />
            <div className="absolute left-[5%] right-[5%] top-0 h-px bg-linear-to-r from-transparent via-amber-600/15 to-transparent" />
            <div className="absolute left-0 bottom-0 h-3 w-3 border-l border-b border-amber-600/20" />
            <div className="absolute right-0 bottom-0 h-3 w-3 border-r border-b border-amber-600/20" />
            <div className="absolute left-[12%] right-[12%] bottom-0 h-px bg-linear-to-r from-transparent via-amber-600/10 to-transparent" />
            {title ? (
                <p
                    className={cn(
                        "mb-4 text-[0.65rem] font-semibold uppercase tracking-[0.2em] text-amber-600/80",
                        titleClassName,
                    )}
                >
                    {title}
                </p>
            ) : null}
            {children}
        </section>
    );
}

export { SectionCard };
