"use client";

import { useEffect } from "react";
import { useRouter } from "next/navigation";

import { useAuth } from "@/stores/auth";

export function AuthGuard({ children }: { children: React.ReactNode }) {
    const router = useRouter();
    const accessToken = useAuth((s) => s.accessToken);

    useEffect(() => {
        if (!accessToken) router.replace("/");
    }, [accessToken, router]);

    if (!accessToken) return null;

    return children;
}
