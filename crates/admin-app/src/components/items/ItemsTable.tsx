import {
    flexRender,
    getCoreRowModel,
    type ColumnDef,
    useReactTable,
} from "@tanstack/react-table";
import { ChevronLeft, ChevronRight, ChevronsUpDown } from "lucide-react";

import type { ItemSummary } from "@/lib/api";
import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "@/components/ui/table";

const rarityColors: Record<string, string> = {
    common: "text-amber-100/50",
    uncommon: "text-emerald-400/70",
    rare: "text-sky-400/70",
    epic: "text-purple-400/70",
};

function RarityBadge({ rarity }: { rarity: string }) {
    return (
        <span
            className={`text-xs font-medium capitalize ${rarityColors[rarity] ?? "text-amber-100/50"}`}
        >
            {rarity}
        </span>
    );
}

const columns: ColumnDef<ItemSummary>[] = [
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
    },
    {
        accessorKey: "rarity",
        header: "Rarity",
        cell: ({ row }) => <RarityBadge rarity={row.getValue("rarity")} />,
    },
    {
        accessorKey: "inventoryType",
        header: "Type",
        cell: ({ row }) => (
            <span className="capitalize">
                {(row.getValue("inventoryType") as string).replace(/_/g, " ")}
            </span>
        ),
    },
    {
        accessorKey: "class",
        header: "Class",
        cell: ({ row }) => (
            <span className="capitalize text-amber-100/55">
                {row.getValue("class") ?? "—"}
            </span>
        ),
    },
    {
        accessorKey: "equipmentSlot",
        header: "Slot",
        cell: ({ row }) => (
            <span className="text-amber-100/55">
                {(row.getValue("equipmentSlot") as string | null) ?? "—"}
            </span>
        ),
    },
    {
        accessorKey: "levelReq",
        header: "Lvl",
        cell: ({ row }) => (
            <span className="text-amber-100/55">
                {(row.getValue("levelReq") as number | null) ?? "—"}
            </span>
        ),
    },
    {
        accessorKey: "slug",
        header: "Slug",
        cell: ({ row }) => (
            <span className="font-mono text-xs text-amber-500/80">
                {row.getValue("slug")}
            </span>
        ),
    },
    {
        accessorKey: "id",
        header: "ID",
        cell: ({ row }) => (
            <span className="font-mono text-xs text-amber-600/65">
                {(row.getValue("id") as string).slice(0, 8)}…
            </span>
        ),
    },
];

export function ItemsTable({
    data,
    page,
    totalPages,
    total,
    isFetching,
    onPageChange,
}: {
    data: ItemSummary[];
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
                                    className="h-16 text-center text-xs text-amber-600/60"
                                >
                                    No items found.
                                </TableCell>
                            </TableRow>
                        )}
                    </TableBody>
                </Table>
            </div>

            <div className="flex items-center justify-between">
                <span className="text-[0.65rem] text-amber-600/70">
                    {isFetching
                        ? "Loading…"
                        : `${total} item${total !== 1 ? "s" : ""} total`}
                </span>
                <div className="flex items-center gap-2">
                    <span className="text-[0.65rem] text-amber-600/70">
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
