import type { Metadata } from "next";
import Link from "next/link";
import type { ReactNode } from "react";

import "./globals.css";

export const metadata: Metadata = {
  title: "Race Agent",
  description: "Sim racing telemetry ingestion and analysis"
};

export default function RootLayout({ children }: { children: ReactNode }) {
  return (
    <html lang="en">
      <body>
        <header className="site-header">
          <Link href="/" style={{ display: "flex", alignItems: "center", gap: "10px" }}>
            <img
              src="/logo.svg"
              alt=""
              width="70"
              height="20"
              style={{ display: "block" }}
            />
            <span>Race Agent</span>
          </Link>
          <nav>
            <Link href="/sessions">Sessions</Link>
          </nav>
        </header>
        <main>{children}</main>
      </body>
    </html>
  );
}
