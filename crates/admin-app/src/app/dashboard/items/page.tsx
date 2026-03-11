"use client";

import { useEffect } from "react";
import { useRouter } from "next/navigation";

import { ItemsView } from "@/components/dashboard/items-view";
import { useAuth } from "@/stores/auth";

export default function ItemsPage() {
    const router = useRouter();
    const accessToken = useAuth((s) => s.accessToken);

    useEffect(() => {
        if (!accessToken) router.replace("/");
    }, [accessToken, router]);

    if (!accessToken) return null;

    return <ItemsView />;
}
