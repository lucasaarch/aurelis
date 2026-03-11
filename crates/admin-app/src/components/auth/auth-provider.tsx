"use client";

import { useEffect, useRef, useState } from "react";
import { usePathname, useRouter } from "next/navigation";

import { useRefreshToken } from "@/hooks/use-auth";
import { LoadingScreen } from "@/components/loading-screen";
import { logoutAuth, useAuth } from "@/stores/auth";

type Phase = "starting" | "refreshing" | "ready";

export function AuthProvider({ children }: { children: React.ReactNode }) {
    const router = useRouter();
    const pathname = usePathname();

    const accessToken = useAuth((s) => s.accessToken);
    const refreshToken = useAuth((s) => s.refreshToken);
    const refreshMutation = useRefreshToken();

    const didRun = useRef(false);
    const [phase, setPhase] = useState<Phase>("starting");

    useEffect(() => {
        if (didRun.current) return;
        didRun.current = true;

        if (!refreshToken) {
            setPhase("ready");
            return;
        }

        setPhase("refreshing");

        refreshMutation
            .mutateAsync({ refreshToken })
            .then(() => setPhase("ready"))
            .catch(() => {
                logoutAuth();
                setPhase("ready");
            });
    }, [refreshMutation, refreshToken]);

    useEffect(() => {
        if (phase !== "ready") return;

        // Routing policy:
        // - Logged in: keep user out of login route.
        // - Logged out: keep user inside login route.
        if (accessToken) {
            if (pathname === "/") router.replace("/dashboard");
            return;
        }

        if (pathname !== "/") router.replace("/");
    }, [accessToken, pathname, phase, router]);

    const showLoading = phase !== "ready";
    const message =
        phase === "starting"
            ? "Starting..."
            : phase === "refreshing"
              ? "Restoring session..."
              : "Loading...";

    return (
        <>
            {phase === "ready" ? children : null}
            <LoadingScreen show={showLoading} message={message} />
        </>
    );
}
