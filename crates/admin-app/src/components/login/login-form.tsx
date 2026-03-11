import { Eye, EyeOff, Loader2 } from "lucide-react";
import { useState } from "react";
import type { UseFormReturn } from "react-hook-form";
import { Controller } from "react-hook-form";
import { Button } from "@/components/ui/button";
import { Field, FieldError, FieldLabel } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { LoginRequest } from "@/lib/api";

interface LoginFormProps {
    form: UseFormReturn<LoginRequest>;
    onSubmit: (data: LoginRequest) => Promise<void>;
}

export function LoginForm({ form, onSubmit }: LoginFormProps) {
    const [showPassword, setShowPassword] = useState(false);
    const isLoading = form.formState.isSubmitting;

    const inputClass = [
        "w-full rounded-sm border bg-[#0a0805] px-3.5 py-2.5",
        "text-sm text-amber-100/80 placeholder:text-amber-900/40",
        "outline-none transition-all duration-200 focus:ring-1",
        "border-amber-600/15 focus:border-amber-600/40 focus:ring-amber-900/20",
        "data-[invalid=true]:border-red-800/60 data-[invalid=true]:focus:border-red-700/60",
    ].join(" ");

    return (
        <form
            onSubmit={form.handleSubmit(onSubmit)}
            className="space-y-5"
            noValidate
        >
            <Controller
                name="email"
                control={form.control}
                render={({ field, fieldState }) => (
                    <Field data-invalid={fieldState.invalid}>
                        <FieldLabel
                            htmlFor={field.name}
                            className="block text-[0.7rem] font-semibold uppercase tracking-[0.15em] text-amber-600/70"
                        >
                            E-mail
                        </FieldLabel>
                        <Input
                            {...field}
                            id={field.name}
                            type="email"
                            placeholder="admin@aurelis.gg"
                            autoComplete="email"
                            aria-invalid={fieldState.invalid}
                            className={inputClass}
                        />
                        {fieldState.invalid && (
                            <FieldError
                                errors={[fieldState.error]}
                                className="text-[0.7rem] text-red-500/70 tracking-wide"
                            />
                        )}
                    </Field>
                )}
            />

            <Controller
                name="password"
                control={form.control}
                render={({ field, fieldState }) => (
                    <Field data-invalid={fieldState.invalid}>
                        <FieldLabel
                            htmlFor={field.name}
                            className="block text-[0.7rem] font-semibold uppercase tracking-[0.15em] text-amber-600/70"
                        >
                            Senha
                        </FieldLabel>
                        <div className="relative">
                            <Input
                                {...field}
                                id={field.name}
                                type={showPassword ? "text" : "password"}
                                placeholder="••••••••"
                                autoComplete="current-password"
                                aria-invalid={fieldState.invalid}
                                className={`${inputClass} pr-10`}
                            />
                            <button
                                type="button"
                                onClick={() => setShowPassword((v) => !v)}
                                className="absolute inset-y-0 right-0 flex items-center px-3 text-amber-800/40 hover:text-amber-600/60 transition-colors"
                                aria-label={
                                    showPassword
                                        ? "Ocultar senha"
                                        : "Mostrar senha"
                                }
                            >
                                {showPassword ? (
                                    <EyeOff className="size-3.5" />
                                ) : (
                                    <Eye className="size-3.5" />
                                )}
                            </button>
                        </div>
                        {fieldState.invalid && (
                            <FieldError
                                errors={[fieldState.error]}
                                className="text-[0.7rem] text-red-500/70 tracking-wide"
                            />
                        )}
                    </Field>
                )}
            />

            <Button
                type="submit"
                disabled={isLoading}
                className={[
                    "relative mt-2 w-full overflow-hidden rounded-sm py-2.5",
                    "border border-amber-600/30 bg-linear-to-b from-amber-900/30 to-amber-950/40",
                    "text-[0.75rem] font-semibold uppercase tracking-[0.2em] text-amber-300/80",
                    "transition-all duration-200",
                    "hover:border-amber-500/50 hover:from-amber-800/30 hover:to-amber-900/40 hover:text-amber-200/90",
                    "hover:shadow-[0_0_20px_rgba(180,120,30,0.15)]",
                    "disabled:cursor-not-allowed disabled:opacity-40",
                    "focus:outline-none focus:ring-1 focus:ring-amber-600/30",
                ].join(" ")}
            >
                <div className="absolute left-[15%] right-[15%] top-0 h-px bg-linear-to-r from-transparent via-amber-500/30 to-transparent" />
                <span className="flex items-center justify-center gap-2">
                    {isLoading && <Loader2 className="size-3.5 animate-spin" />}
                    {isLoading ? "Entrando…" : "Entrar"}
                </span>
            </Button>
        </form>
    );
}
