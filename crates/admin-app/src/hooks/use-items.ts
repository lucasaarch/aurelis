import { AdminService } from "@/lib/api";
import { useQuery } from "@tanstack/react-query";

export function useItems(params: { page: number; limit: number }) {
    return useQuery({
        queryKey: ["items", params.page, params.limit],
        queryFn: () => AdminService.listItems(params),
    });
}
