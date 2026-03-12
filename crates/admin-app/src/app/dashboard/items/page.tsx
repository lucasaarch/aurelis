"use client";

import { useMemo, useState } from "react";
import { useForm } from "react-hook-form";

import { ItemsTable } from "@/components/items/items-table";
import {
    ItemSearchForm,
    type ItemSearchValues,
} from "@/components/items/item-search-form";
import { PageTitle } from "@/components/page-title";
import { SectionCard } from "@/components/section-card";
import { useItems } from "@/hooks/use-items";
import type { ItemsQuery } from "@/hooks/use-items";

const LIMIT = 20;

export default function ItemsPage() {
    const [page, setPage] = useState(1);
    const [filters, setFilters] = useState<ItemsQuery>({});
    const filterForm = useForm<ItemSearchValues>({
        defaultValues: {
            search: "",
            rarity: "_all",
            inventoryType: "_all",
            class: "_all",
            equipmentSlot: "_all",
            levelMin: "",
            levelMax: "",
        },
    });
    const params = useMemo<ItemsQuery>(
        () => ({
            page,
            limit: LIMIT,
            ...filters,
        }),
        [page, filters],
    );

    const { data, isFetching, isError } = useItems(params);
    function applyFilters(values: ItemSearchValues) {
        setPage(1);
        setFilters({
            search: values.search || undefined,
            rarity: values.rarity === "_all" ? undefined : values.rarity,
            inventoryType:
                values.inventoryType === "_all"
                    ? undefined
                    : values.inventoryType,
            class: values.class === "_all" ? undefined : values.class,
            equipmentSlot:
                values.equipmentSlot === "_all"
                    ? undefined
                    : values.equipmentSlot,
            levelMin: values.levelMin ? Number(values.levelMin) : undefined,
            levelMax: values.levelMax ? Number(values.levelMax) : undefined,
        });
    }

    return (
        <div className="space-y-6">
            <PageTitle title="Item Management" />

            <div className="h-px bg-linear-to-r from-amber-600/15 via-amber-600/5 to-transparent" />

            <SectionCard>
                <ItemSearchForm form={filterForm} onSubmit={applyFilters} />

                <div className="pt-4" />

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
