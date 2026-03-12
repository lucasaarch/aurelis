import {
    flexRender,
    getCoreRowModel,
    type ColumnDef,
    useReactTable,
} from "@tanstack/react-table";
import { ChevronLeft, ChevronRight, ChevronsUpDown } from "lucide-react";
import { toast } from "sonner";

import type { MobSummary } from "@/lib/api";
import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "@/components/ui/table";
const typeColors: Record<string, string> = {
    common: "text-amber-100/55",
    miniboss: "text-amber-300/70",
    boss: "text-red-400/70",
    raidboss: "text-red-500/75",
};

const columns: ColumnDef<MobSummary>[] = [
    {
        accessorKey: "slug",
        header: "Slug",
        cell: ({ row }) => {
            const slug = row.getValue("slug") as string;
            return (
                <button
                    type="button"
                    onClick={async () => {
                        try {
                            await navigator.clipboard.writeText(slug);
                            toast.success("Slug copied successfully.");
                        } catch {
                            toast.error("Failed to copy slug.");
                        }
                    }}
                    className="cursor-pointer text-amber-500/80 transition-colors hover:text-amber-300/90"
                    aria-label={`Copy slug ${slug}`}
                >
                    {slug}
                </button>
            );
        },
    },
    {
        accessorKey: "name",
        header: ({ column }) => (
            <button
                className="flex cursor-pointer items-center gap-1 uppercase tracking-[0.18em]"
                onClick={() =>
                    column.toggleSorting(column.getIsSorted() === "asc")
                }
            >
                Name
                <ChevronsUpDown className="size-3 opacity-50" />
            </button>
        ),
        cell: ({ row }) => (
            <span className="text-amber-100/85">{row.getValue("name")}</span>
        ),
    },
    {
        accessorKey: "mobType",
        header: "Type",
        cell: ({ row }) => (
            <span
                className={`capitalize ${typeColors[row.getValue("mobType") as string] ?? "text-amber-100/55"}`}
            >
                {(row.getValue("mobType") as string).replace(/_/g, " ")}
            </span>
        ),
    },
    {
        accessorKey: "description",
        header: "Description",
        cell: ({ row }) => (
            <span className="text-amber-100/55">
                {(row.getValue("description") as string | null) ?? "—"}
            </span>
        ),
    },
    {
        accessorKey: "createdAt",
        header: "Created",
        cell: ({ row }) => {
            const createdAt = row.getValue("createdAt") as string;
            const formatted = new Date(createdAt).toLocaleDateString("en-US", {
                year: "numeric",
                month: "short",
                day: "2-digit",
            });
            return <span className="text-amber-100/55">{formatted}</span>;
        },
    },
];

export function MobsTable({
    data,
    page,
    totalPages,
    total,
    isFetching,
    onPageChange,
}: {
    data: MobSummary[];
    page: number;
    totalPages: number;
    total: number;
    isFetching: boolean;
    onPageChange: (page: number) => void;
}) {
    const table = useReactTable({
        data,
        columns,
        getCoreRowModel: getCoreRowModel(),
        manualSorting: true,
    });

    return (
        <div className="space-y-3">
            <div className="overflow-hidden rounded-sm border border-amber-600/10">
                <Table>
                    <TableHeader>
                        {table.getHeaderGroups().map((hg) => (
                            <TableRow key={hg.id}>
                                {hg.headers.map((h) => (
                                    <TableHead key={h.id}>
                                        {h.isPlaceholder
                                            ? null
                                            : flexRender(
                                                  h.column.columnDef.header,
                                                  h.getContext(),
                                              )}
                                    </TableHead>
                                ))}
                            </TableRow>
                        ))}
                    </TableHeader>
                    <TableBody>
                        {table.getRowModel().rows.length ? (
                            table.getRowModel().rows.map((row) => (
                                <TableRow key={row.id}>
                                    {row.getVisibleCells().map((cell) => (
                                        <TableCell key={cell.id}>
                                            {flexRender(
                                                cell.column.columnDef.cell,
                                                cell.getContext(),
                                            )}
                                        </TableCell>
                                    ))}
                                </TableRow>
                            ))
                        ) : (
                            <TableRow>
                                <TableCell
                                    colSpan={columns.length}
                                    className="h-16 text-center text-amber-600/60"
                                >
                                    No mobs found.
                                </TableCell>
                            </TableRow>
                        )}
                    </TableBody>
                </Table>
            </div>

            <div className="flex items-center justify-between">
                <span className="text-xs text-amber-600/70">
                    {isFetching
                        ? "Loading…"
                        : `${total} mob${total !== 1 ? "s" : ""} total`}
                </span>
                <div className="flex items-center gap-2">
                    <span className="text-xs text-amber-600/70">
                        Page {page} of {totalPages || 1}
                    </span>
                    <button
                        onClick={() => onPageChange(page - 1)}
                        disabled={page <= 1 || isFetching}
                        className="flex size-6 cursor-pointer items-center justify-center rounded-sm border border-amber-600/30 text-amber-500/80 transition-colors hover:border-amber-500/60 hover:text-amber-400 disabled:cursor-not-allowed disabled:opacity-30"
                    >
                        <ChevronLeft className="size-3.5" />
                    </button>
                    <button
                        onClick={() => onPageChange(page + 1)}
                        disabled={page >= totalPages || isFetching}
                        className="flex size-6 cursor-pointer items-center justify-center rounded-sm border border-amber-600/30 text-amber-500/80 transition-colors hover:border-amber-500/60 hover:text-amber-400 disabled:cursor-not-allowed disabled:opacity-30"
                    >
                        <ChevronRight className="size-3.5" />
                    </button>
                </div>
            </div>
        </div>
    );
}
