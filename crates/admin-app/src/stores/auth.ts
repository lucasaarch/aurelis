"use client";

import { create } from "zustand";
import { OpenAPI } from "@/lib/api";

export interface User {
    id: string;
    email: string;
    name: string;
}

interface AuthState {
    user: User | null;
    accessToken: string | null;
    refreshToken: string | null;
    setTokens: (
        accessToken: string | null,
        refreshToken?: string | null,
    ) => void;
    setUser: (user: User | null) => void;
    logout: () => void;
}

const storage = {
    get(key: string): string | null {
        try {
            return localStorage.getItem(key);
        } catch {
            return null;
        }
    },
    set(key: string, value: string) {
        try {
            localStorage.setItem(key, value);
        } catch {
            // ignore (private mode/quota/etc)
        }
    },
    remove(key: string) {
        try {
            localStorage.removeItem(key);
        } catch {
            // ignore
        }
    },
};

const initialAccess = storage.get("accessToken");
const initialRefresh = storage.get("refreshToken");
const initialUser = (() => {
    const raw = storage.get("user");
    if (!raw) return null;
    try {
        return JSON.parse(raw) as User;
    } catch {
        return null;
    }
})();

// Initialize OpenAPI.TOKEN if we have an access token available at startup.
// This makes the generated client automatically include the Authorization header.
if (initialAccess) {
    OpenAPI.TOKEN = async () => initialAccess;
} else {
    OpenAPI.TOKEN = undefined;
}

export const useAuth = create<AuthState>((set) => ({
    user: initialUser,
    accessToken: initialAccess,
    refreshToken: initialRefresh,
    setTokens: (accessToken, refreshToken = null) => {
        if (accessToken === null) storage.remove("accessToken");
        else storage.set("accessToken", accessToken);

        if (refreshToken === null) storage.remove("refreshToken");
        else if (refreshToken !== undefined)
            storage.set("refreshToken", refreshToken);

        // Wire OpenAPI token resolver so the generated client uses the current access token.
        if (accessToken === null) OpenAPI.TOKEN = undefined;
        else OpenAPI.TOKEN = async () => accessToken;

        set({
            accessToken,
            refreshToken: refreshToken ?? null,
        });
    },
    setUser: (user) => {
        if (user === null) storage.remove("user");
        else storage.set("user", JSON.stringify(user));

        set({ user });
    },
    logout: () => {
        storage.remove("accessToken");
        storage.remove("refreshToken");
        storage.remove("user");

        // Ensure the generated API client stops attaching Authorization.
        OpenAPI.TOKEN = undefined;

        set({
            user: null,
            accessToken: null,
            refreshToken: null,
        });
    },
}));

// Imperative helpers for legacy hooks or non-React usage.
// These allow the existing react-query login hook to call into the store
// instead of manipulating localStorage directly.
export const setAuthTokens = (
    accessToken: string | null,
    refreshToken?: string | null,
) => {
    useAuth.getState().setTokens(accessToken, refreshToken ?? null);
};

export const setAuthUser = (user: User | null) => {
    useAuth.getState().setUser(user);
};

export const logoutAuth = () => {
    useAuth.getState().logout();
};
