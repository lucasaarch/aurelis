"use client";

import { AnimatePresence, motion } from "framer-motion";
import { Loader2 } from "lucide-react";

import { cn } from "@/lib/utils";

type LoadingScreenProps = {
    show: boolean;
    message?: string;
    className?: string;
};

export function LoadingScreen({
    show,
    message = "Loading",
    className,
}: LoadingScreenProps) {
    return (
        <AnimatePresence>
            {show && (
                <motion.div
                    data-slot="loading-screen"
                    className={cn(
                        "fixed inset-0 z-50 grid place-items-center bg-[#080a0f]",
                        className,
                    )}
                    initial={{ opacity: 0 }}
                    animate={{ opacity: 1 }}
                    exit={{ opacity: 0 }}
                    transition={{ duration: 0.18, ease: "easeOut" }}
                >
                    <div className="pointer-events-none absolute inset-0">
                        <div className="absolute left-1/2 top-0 h-100 w-175 -translate-x-1/2 rounded-full bg-amber-600/10 blur-[120px]" />
                        <div className="absolute bottom-0 left-1/4 h-75 w-100 rounded-full bg-amber-900/8 blur-[100px]" />
                    </div>

                    <motion.div
                        className="relative flex items-center gap-3 text-amber-600/80"
                        initial={{ opacity: 0, y: 6 }}
                        animate={{ opacity: 1, y: 0 }}
                        exit={{ opacity: 0, y: 6 }}
                        transition={{ duration: 0.2, ease: "easeOut" }}
                    >
                        <Loader2 className="size-4 animate-spin" />
                        <span className="text-xs uppercase tracking-[0.2em]">
                            {message}
                        </span>
                    </motion.div>
                </motion.div>
            )}
        </AnimatePresence>
    );
}
