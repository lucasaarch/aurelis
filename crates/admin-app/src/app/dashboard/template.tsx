"use client";

import { useRouter } from "next/navigation";
import { useEffect } from "react";

import { useAuth } from "@/stores/auth";

export default function DashboardTemplate({
    children,
}: Readonly<{
    children: React.ReactNode;
}>) {
    const router = useRouter();
    const accessToken = useAuth((s) => s.accessToken);

    useEffect(() => {
        if (!accessToken) router.replace("/");
    }, [accessToken, router]);

    if (!accessToken) return null;

    return <>{children}</>;
}
