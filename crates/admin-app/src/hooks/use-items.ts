import { AdminService } from "@/lib/api";
import { useQuery } from "@tanstack/react-query";

export type ItemsQuery = {
    page?: number;
    limit?: number;
    class?: string;
    rarity?: string;
    equipmentSlot?: string;
    inventoryType?: string;
    levelMin?: number;
    levelMax?: number;
    search?: string;
};

export function useItems(params: ItemsQuery) {
    return useQuery({
        queryKey: ["items", params],
        queryFn: () =>
            AdminService.listItems({
                page: params.page,
                limit: params.limit,
                _class: params.class,
                rarity: params.rarity,
                equipmentSlot: params.equipmentSlot,
                inventoryType: params.inventoryType,
                levelMin: params.levelMin,
                levelMax: params.levelMax,
                search: params.search,
            }),
    });
}
