"use client";

import { useMemo, useState } from "react";
import { useForm } from "react-hook-form";

import { MobsTable } from "@/components/mobs/mobs-table";
import { PageTitle } from "@/components/page-title";
import { SectionCard } from "@/components/section-card";
import { FieldItem } from "@/components/ui/field";
import { FormSelect } from "@/components/ui/form-select";
import { Input } from "@/components/ui/input";
import { useMobs, type MobsQuery } from "@/hooks/use-mobs";
import { displayEnum } from "@/lib/utils";
import { Button } from "@/components/ui/button";

const LIMIT = 20;
const mobTypes = ["common", "miniboss", "boss", "raidboss"] as const;
const mobTypeOptions = [
    { value: "_all", label: "All" },
    ...mobTypes.map((type) => ({
        value: type,
        label: displayEnum(type),
    })),
];

type FilterValues = {
    search: string;
    mobType: string;
};

export default function MobsPage() {
    const [page, setPage] = useState(1);
    const [filters, setFilters] = useState<MobsQuery>({});
    const filterForm = useForm<FilterValues>({
        defaultValues: {
            search: "",
            mobType: "_all",
        },
    });

    const params = useMemo<MobsQuery>(
        () => ({
            page,
            limit: LIMIT,
            ...filters,
        }),
        [page, filters],
    );

    const { data, isFetching, isError } = useMobs(params);
    function applyFilters(values: FilterValues) {
        setPage(1);
        setFilters({
            search: values.search || undefined,
            mobType: values.mobType === "_all" ? undefined : values.mobType,
        });
    }

    return (
        <div className="space-y-6">
            <PageTitle title="Mob Management" />

            <div className="h-px bg-linear-to-r from-amber-600/15 via-amber-600/5 to-transparent" />

            <SectionCard>
                <form
                    onSubmit={filterForm.handleSubmit(applyFilters)}
                    className="space-y-4 pb-2"
                >
                    <div className="grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
                        <FieldItem label="Search">
                            <Input
                                {...filterForm.register("search")}
                                placeholder="Search by name"
                            />
                        </FieldItem>

                        <FieldItem label="Type">
                            <FormSelect
                                control={filterForm.control}
                                name="mobType"
                                options={mobTypeOptions}
                                placeholder="All"
                            />
                        </FieldItem>
                    </div>

                    <div className="flex items-center gap-3">
                        <Button type="submit" variant="resonaSubmit">
                            Apply filters
                        </Button>
                    </div>
                </form>

                <div className="pt-4" />

                {isError ? (
                    <p className="text-xs text-red-500/60">
                        Failed to load mobs.
                    </p>
                ) : (
                    <MobsTable
                        data={data?.mobs ?? []}
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
