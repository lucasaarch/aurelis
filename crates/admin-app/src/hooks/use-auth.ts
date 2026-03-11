"use client";

import { useMutation } from "@tanstack/react-query";
import { AuthService } from "@/lib/api";
import type {
    LoginRequest,
    LoginResponse,
    RefreshRequest,
    RefreshResponse,
} from "@/lib/api";
import { setAuthTokens, setAuthUser } from "@/stores/auth";

export function useLogin() {
    return useMutation<LoginResponse, unknown, LoginRequest>({
        mutationFn: async (data: LoginRequest) => {
            const response = await AuthService.login({
                requestBody: data,
            });
            return response;
        },
        onSuccess: (data) => {
            try {
                setAuthTokens(data.accessToken, data.refreshToken);
            } catch (e) {
                console.warn("Failed to set auth tokens in store", e);
            }

            const maybeUser = (data as any).user;
            if (maybeUser) {
                try {
                    setAuthUser(maybeUser);
                } catch (e) {
                    console.warn("Failed to set auth user in store", e);
                }
            }
        },
    });
}

export function useRefreshToken() {
    return useMutation<RefreshResponse, unknown, RefreshRequest>({
        mutationFn: async (data: RefreshRequest) => {
            const response = await AuthService.refreshToken({
                requestBody: data,
            });
            return response;
        },
        onSuccess: (data) => {
            try {
                setAuthTokens(data.accessToken, data.refreshToken);
            } catch (e) {
                console.warn("Failed to set auth tokens in store", e);
            }

            const maybeUser = (data as any).user;
            if (maybeUser) {
                try {
                    setAuthUser(maybeUser);
                } catch (e) {
                    console.warn("Failed to set auth user in store", e);
                }
            }
        },
    });
}
