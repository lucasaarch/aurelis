"use client";

import { AuthProvider } from "@/components/auth/auth-provider";

export default function Template({
    children,
}: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <AuthProvider>
            <div className="w-full h-[calc(100vh-32px)]">{children}</div>
        </AuthProvider>
    );
}
