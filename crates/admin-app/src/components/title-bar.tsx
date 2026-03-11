"use client";

import { Minus, Square, X } from "lucide-react";
import { getCurrentWindow, type Window } from "@tauri-apps/api/window";
import { useEffect, useState } from "react";

export function TitleBar() {
    const [win, setWin] = useState<Window | null>(null);

    // Next "use client" components can still render on the server for the initial HTML.
    // Accessing Tauri's window APIs must happen only after the component mounts.
    useEffect(() => {
        try {
            setWin(getCurrentWindow());
        } catch {
            setWin(null);
        }
    }, []);

    return (
        <div
            data-tauri-drag-region
            className="sticky left-0 right-0 top-0 z-50 flex h-8 items-center justify-between border-b border-amber-600/10 bg-[#080a0f]/90 backdrop-blur-sm"
        >
            <div className="absolute bottom-0 left-[5%] right-[5%] h-px bg-linear-to-r from-transparent via-amber-600/15 to-transparent" />

            <span
                data-tauri-drag-region
                className="pl-3 text-[0.6rem] font-semibold uppercase tracking-[0.25em] text-amber-700/40 select-none"
            >
                Resona
            </span>

            <div className="flex h-full items-center">
                <button
                    onClick={() => win?.minimize()}
                    disabled={!win}
                    className="group cursor-pointer flex h-full w-10 items-center justify-center text-amber-800/30 transition-all duration-200 hover:bg-amber-600/5 hover:text-amber-500/60 disabled:pointer-events-none disabled:opacity-40"
                    aria-label="Minimizar"
                >
                    <Minus className="size-3" />
                </button>

                <button
                    onClick={() => win?.toggleMaximize()}
                    disabled={!win}
                    className="group cursor-pointer flex h-full w-10 items-center justify-center text-amber-800/30 transition-all duration-200 hover:bg-amber-600/5 hover:text-amber-500/60 disabled:pointer-events-none disabled:opacity-40"
                    aria-label="Maximizar"
                >
                    <Square className="size-2.5" />
                </button>

                <button
                    onClick={() => win?.close()}
                    disabled={!win}
                    className="group cursor-pointer flex h-full w-10 items-center justify-center text-amber-800/30 transition-all duration-200 hover:bg-red-900/20 hover:text-red-400/60 disabled:pointer-events-none disabled:opacity-40"
                    aria-label="Fechar"
                >
                    <X className="size-3.5" />
                </button>
            </div>
        </div>
    );
}
