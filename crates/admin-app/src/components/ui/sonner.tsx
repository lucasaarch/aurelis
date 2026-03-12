"use client";

import { useTheme } from "next-themes";
import { Toaster as Sonner, type ToasterProps } from "sonner";
import {
    CircleCheckIcon,
    InfoIcon,
    TriangleAlertIcon,
    OctagonXIcon,
    Loader2Icon,
} from "lucide-react";

const Toaster = ({ ...props }: ToasterProps) => {
    const { theme = "system" } = useTheme();
    const cornerStyle: React.CSSProperties = {
        backgroundColor: "rgba(12,10,6,0.98)",
        backgroundImage:
            "linear-gradient(to bottom, rgba(18,16,10,0.98), rgba(12,10,6,0.98))," +
            "linear-gradient(to right, rgba(180,120,30,0.65), rgba(180,120,30,0.65))," +
            "linear-gradient(to bottom, rgba(180,120,30,0.65), rgba(180,120,30,0.65))," +
            "linear-gradient(to left, rgba(180,120,30,0.65), rgba(180,120,30,0.65))," +
            "linear-gradient(to bottom, rgba(180,120,30,0.65), rgba(180,120,30,0.65))," +
            "linear-gradient(to right, rgba(180,120,30,0.65), rgba(180,120,30,0.65))," +
            "linear-gradient(to top, rgba(180,120,30,0.65), rgba(180,120,30,0.65))," +
            "linear-gradient(to left, rgba(180,120,30,0.65), rgba(180,120,30,0.65))," +
            "linear-gradient(to top, rgba(180,120,30,0.65), rgba(180,120,30,0.65))",
        backgroundSize:
            "100% 100%," +
            "14px 1px, 1px 14px," +
            "14px 1px, 1px 14px," +
            "14px 1px, 1px 14px," +
            "14px 1px, 1px 14px",
        backgroundPosition:
            "0 0," +
            "0 0, 0 0," +
            "100% 0, 100% 0," +
            "0 100%, 0 100%," +
            "100% 100%, 100% 100%",
        backgroundRepeat: "no-repeat",
    };

    return (
        <Sonner
            theme={theme as ToasterProps["theme"]}
            className="toaster group"
            icons={{
                success: <CircleCheckIcon className="size-4" />,
                info: <InfoIcon className="size-4" />,
                warning: <TriangleAlertIcon className="size-4" />,
                error: <OctagonXIcon className="size-4" />,
                loading: <Loader2Icon className="size-4 animate-spin" />,
            }}
            style={
                {
                    "--normal-bg": "transparent",
                    "--normal-text": "var(--popover-foreground)",
                    "--normal-border": "transparent",
                    "--border-radius": "var(--radius-sm)",
                } as React.CSSProperties
            }
            toastOptions={{
                style: {
                    ...cornerStyle,
                    display: "flex",
                    alignItems: "center",
                    gap: "0.75rem",
                    padding: "0.5rem 0.75rem",
                    border: "1px solid rgba(180,120,30,0.35)",
                    borderRadius: "0px",
                    color: "rgba(180,120,30,0.7)",
                    fontSize: "0.6rem",
                    lineHeight: "1.1rem",
                    boxShadow:
                        "0 12px 30px rgba(0,0,0,0.5), inset 0 0 60px rgba(180,140,50,0.03)",
                    backdropFilter: "blur(10px)",
                },
                classNames: {
                    title: "text-[0.6rem] font-semibold uppercase tracking-[0.18em] text-amber-600/80",
                    description: "text-[0.6rem] text-amber-600/70",
                    icon: "text-amber-600/80",
                    actionButton:
                        "rounded-md border border-amber-500/30 bg-amber-900/30 px-2 py-1 text-[0.65rem] font-semibold uppercase tracking-[0.2em] text-amber-100/80 transition-colors hover:border-amber-400/60 hover:text-amber-100",
                    cancelButton:
                        "rounded-md border border-transparent bg-transparent px-2 py-1 text-[0.65rem] uppercase tracking-[0.2em] text-amber-200/50 transition-colors hover:text-amber-200/70",
                    closeButton:
                        "text-amber-500/60 transition-colors hover:text-amber-300/90",
                },
            }}
            {...props}
        />
    );
};

export { Toaster };
