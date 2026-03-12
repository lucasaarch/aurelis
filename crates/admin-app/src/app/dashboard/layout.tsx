import { AuthGuard } from "@/components/auth/auth-guard";
import { Sidebar } from "@/components/sidebar";

export default function DashboardLayout({
    children,
}: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <AuthGuard>
            <div className="relative flex h-full flex-col overflow-hidden bg-[#080a0f]">
                <div className="pointer-events-none absolute inset-0">
                    <div className="absolute left-1/2 top-0 h-75 w-150 -translate-x-1/2 rounded-full bg-amber-600/5 blur-[100px]" />
                </div>

                <div className="flex h-full">
                    <Sidebar />

                    <main className="flex flex-1 flex-col overflow-auto p-6 gap-6">
                        {children}
                    </main>
                </div>
            </div>
        </AuthGuard>
    );
}
