"use client";

import { TitleBar } from "@/components/title-bar";
import { AuthProvider } from "@/components/auth/auth-provider";

export default function Template({
    children,
}: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <>
            <TitleBar />
            <div className="w-full h-[calc(100vh-32px)]">
                <AuthProvider>{children}</AuthProvider>
            </div>
        </>
    );
}
