"use client";

import { standardSchemaResolver } from "@hookform/resolvers/standard-schema";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { CorneredPanel } from "@/components/cornered-panel";
import { LoginForm } from "@/components/login/login-form";
import { useLogin } from "@/hooks/use-auth";
import { LoginRequest } from "@/lib/api";
import Image from "next/image";
import { useRouter } from "next/navigation";

const loginSchema = z.object({
    email: z.email("Invalid email address."),
    password: z.string().min(1, "Password is required."),
});

export default function LoginPage() {
    const router = useRouter();
    const loginMutation = useLogin();
    const form = useForm<LoginRequest>({
        resolver: standardSchemaResolver(loginSchema),
        defaultValues: { email: "", password: "" },
    });

    async function onSubmit(data: LoginRequest) {
        try {
            await loginMutation.mutateAsync(data);
            router.replace("/dashboard");
        } catch (err) {
            console.error("Login failed", err);
        }
    }

    return (
        <div className="relative flex h-full items-center justify-center overflow-hidden bg-[#080a0f] px-4">
            <div className="pointer-events-none absolute inset-0">
                <div className="absolute left-1/2 top-0 h-100 w-175 -translate-x-1/2 rounded-full bg-amber-600/10 blur-[120px]" />
                <div className="absolute bottom-0 left-1/4 h-75 w-100 rounded-full bg-amber-900/8 blur-[100px]" />
            </div>

            <div className="relative z-10 w-full max-w-sm space-y-8">
                <div className="flex flex-col items-center gap-3">
                    <div className="relative flex size-16 items-center justify-center">
                        <div className="absolute inset-0 rounded-full bg-amber-500/10 blur-xl" />
                        <svg
                            viewBox="0 0 64 64"
                            fill="none"
                            className="relative size-14 drop-shadow-[0_0_12px_rgba(200,160,60,0.5)]"
                        >
                            <polygon
                                points="32,4 60,52 4,52"
                                stroke="rgba(200,160,60,0.8)"
                                strokeWidth="1.5"
                                fill="rgba(200,160,60,0.06)"
                            />
                            <polygon
                                points="32,18 50,48 14,48"
                                stroke="rgba(200,160,60,0.3)"
                                strokeWidth="0.75"
                                fill="none"
                            />
                            <circle
                                cx="32"
                                cy="32"
                                r="6"
                                stroke="rgba(200,160,60,0.6)"
                                strokeWidth="1"
                                fill="rgba(200,160,60,0.15)"
                            />
                        </svg>
                    </div>

                    <Image
                        src="/logo-login.svg"
                        alt="Resona"
                        className="w-60 object-contain drop-shadow-[0_0_20px_rgba(200,160,60,0.4)]"
                        style={{ height: "calc(240px * 18367 / 55679)" }}
                        width={556.79}
                        height={183.67}
                    />
                </div>

                <CorneredPanel>
                    <div className="mb-5">
                        <p className="text-[0.7rem] font-semibold uppercase tracking-[0.2em] text-amber-600/70">
                            Restricted access
                        </p>
                        <p className="mt-1 text-sm font-light italic text-amber-200/30">
                            Enter your credentials to continue.
                        </p>
                    </div>

                    <div className="mb-5 h-px bg-linear-to-r from-transparent via-amber-600/15 to-transparent" />

                    <LoginForm form={form} onSubmit={onSubmit} />
                </CorneredPanel>
            </div>
        </div>
    );
}
