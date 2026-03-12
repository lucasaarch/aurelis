"use client";

import type * as React from "react";

import { cn } from "@/lib/utils";

export function PageTitle({
    title,
    actions,
    className,
}: {
    title: string;
    actions?: React.ReactNode;
    className?: string;
}) {
    return (
        <div className={cn("flex items-center justify-between", className)}>
            <h1 className="font-serif text-2xl font-bold tracking-wide text-amber-100/90">
                {title}
            </h1>
            {actions ? <div className="flex items-center gap-2">{actions}</div> : null}
        </div>
    );
}
