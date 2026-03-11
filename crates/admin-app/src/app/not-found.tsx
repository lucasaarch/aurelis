import Link from "next/link";
import { Button } from "@/components/ui/button";

export default function NotFoundPage() {
    return (
        <div className="relative flex min-h-dvh items-center justify-center overflow-hidden bg-[#080a0f] px-6 py-12">
            <div className="pointer-events-none absolute inset-0">
                <div className="absolute left-1/2 top-0 h-100 w-200 -translate-x-1/2 rounded-full bg-amber-600/10 blur-[130px]" />
                <div className="absolute bottom-0 left-[15%] h-75 w-140 rounded-full bg-amber-900/10 blur-[120px]" />
            </div>

            <div className="relative z-10 w-full max-w-2xl">
                <div className="relative overflow-hidden rounded-sm border border-amber-600/20 bg-linear-to-b from-[#12100a]/95 to-[#0c0a06]/98 p-8 shadow-[0_24px_70px_rgba(0,0,0,0.7),inset_0_0_80px_rgba(180,140,50,0.02)]">
                    <div className="absolute left-[12%] right-[12%] top-0 h-px bg-linear-to-r from-transparent via-amber-500/50 to-transparent" />
                    <div className="absolute bottom-0 left-[22%] right-[22%] h-px bg-linear-to-r from-transparent via-amber-600/20 to-transparent" />

                    <div className="absolute left-0 top-0 h-3 w-3 border-l border-t border-amber-500/40" />
                    <div className="absolute right-0 top-0 h-3 w-3 border-r border-t border-amber-500/40" />
                    <div className="absolute bottom-0 left-0 h-3 w-3 border-b border-l border-amber-500/40" />
                    <div className="absolute bottom-0 right-0 h-3 w-3 border-b border-r border-amber-500/40" />

                    <div>
                        <p className="text-[0.7rem] font-semibold uppercase tracking-[0.3em] text-amber-600/70">
                            Error 404
                        </p>
                        <h1 className="mt-3 text-3xl font-semibold text-amber-100/90 sm:text-4xl">
                            Page not found
                        </h1>
                        <p className="mt-3 max-w-xl text-sm text-amber-200/40">
                            The requested path does not exist or has been moved.
                            Check the URL or return to a safe section of the
                            panel.
                        </p>
                    </div>

                    <div className="mt-8 flex flex-col gap-3 sm:flex-row sm:items-center">
                        <Button asChild variant="resonaSubmit" size="lg">
                            <Link href="/">Go to login</Link>
                        </Button>
                        <Button
                            asChild
                            variant="outline"
                            size="lg"
                            className="border-amber-500/40 text-amber-100/80"
                        >
                            <Link href="/dashboard">Go to dashboard</Link>
                        </Button>
                    </div>
                </div>
            </div>
        </div>
    );
}
