"use client";

import {
    Activity,
    Package,
    ScrollText,
    Shield,
    Sword,
    TrendingUp,
    Users,
    Zap,
} from "lucide-react";
import Link from "next/link";
import { usePathname } from "next/navigation";

const navItems: {
    icon: React.ElementType;
    label: string;
    href: string;
}[] = [
    { icon: Activity, label: "Dashboard", href: "/dashboard" },
    { icon: Users, label: "Contas", href: "/dashboard/accounts" },
    { icon: Sword, label: "Personagens", href: "/dashboard/characters" },
    { icon: Package, label: "Itens", href: "/dashboard/items" },
    { icon: ScrollText, label: "Quests", href: "/dashboard/quests" },
    { icon: Shield, label: "Mobs", href: "/dashboard/mobs" },
    { icon: Zap, label: "Dungeons", href: "/dashboard/dungeons" },
    { icon: TrendingUp, label: "Relatórios", href: "/dashboard/reports" },
];

export function Sidebar() {
    const pathname = usePathname();

    return (
        <aside className="relative flex w-14 flex-col items-center border-r border-amber-600/10 bg-[#080a0f] py-4 gap-1">
            <div className="absolute right-0 top-[10%] bottom-[10%] w-px bg-linear-to-b from-transparent via-amber-600/10 to-transparent" />

            {navItems.map(({ icon: Icon, label, href }) => {
                const active = pathname === href;
                return (
                    <Link
                        key={label}
                        title={label}
                        href={href}
                        aria-current={active ? "page" : undefined}
                        className={[
                            "group relative flex size-9 cursor-pointer items-center justify-center rounded-sm transition-all duration-200",
                            active
                                ? "border border-amber-600/25 bg-amber-900/15 text-amber-500/80"
                                : "text-amber-800/40 hover:bg-amber-900/10 hover:text-amber-600/60",
                        ].join(" ")}
                    >
                        {active && (
                            <div className="absolute right-0 top-1/2 h-4 w-px -translate-y-1/2 bg-amber-500/50" />
                        )}
                        <Icon className="size-3.5" />
                    </Link>
                );
            })}
        </aside>
    );
}
