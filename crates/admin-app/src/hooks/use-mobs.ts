import { useQuery } from "@tanstack/react-query";

import { AdminService } from "@/lib/api";

export type MobsQuery = {
    page?: number;
    limit?: number;
    mobType?: string;
    search?: string;
};

export function useMobs(params: MobsQuery) {
    return useQuery({
        queryKey: ["mobs", params],
        queryFn: () =>
            AdminService.listMobs({
                page: params.page,
                limit: params.limit,
                mobType: params.mobType,
                search: params.search,
            }),
    });
}

// mutations removed
