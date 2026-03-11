"use client";

import { useState } from "react";
import { useQueryClient } from "@tanstack/react-query";

import { CreateItemForm } from "@/components/items/create-item-form";
import { GiveItemForm } from "@/components/items/give-item-form";
import { ItemsTable } from "@/components/items/ItemsTable";
import { SectionCard } from "@/components/section-card";
import { useItems } from "@/hooks/use-items";

const LIMIT = 20;

export function ItemsView() {
    const [page, setPage] = useState(1);
    const queryClient = useQueryClient();
    const { data, isFetching, isError } = useItems({ page, limit: LIMIT });

    function handleCreated() {
        setPage(1);
        queryClient.invalidateQueries({ queryKey: ["items"] });
    }

    return (
        <div className="space-y-6">
            <div className="flex items-end justify-between">
                <div>
                    <p className="text-[0.6rem] font-semibold uppercase tracking-[0.25em] text-amber-700/40">
                        Resona · Admin
                    </p>
                    <h1 className="font-serif text-xl font-bold tracking-wide text-amber-100/80">
                        Dashboard
                    </h1>
                </div>
            </div>

            <div className="h-px bg-linear-to-r from-amber-600/15 via-amber-600/5 to-transparent" />

            <SectionCard title="Create item">
                <CreateItemForm onSuccess={handleCreated} />
            </SectionCard>

            <SectionCard title="Give item to character">
                <GiveItemForm />
            </SectionCard>

            <SectionCard title="Items">
                {isError ? (
                    <p className="text-xs text-red-500/60">
                        Failed to load items.
                    </p>
                ) : (
                    <ItemsTable
                        data={data?.items ?? []}
                        page={data?.page ?? page}
                        totalPages={data?.totalPages ?? 1}
                        total={data?.total ?? 0}
                        isFetching={isFetching}
                        onPageChange={setPage}
                    />
                )}
            </SectionCard>
        </div>
    );
}
