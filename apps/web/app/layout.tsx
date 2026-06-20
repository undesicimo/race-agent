import type { Metadata } from "next";
import Link from "next/link";
import type { ReactNode } from "react";

import "./globals.css";

export const metadata: Metadata = {
  title: "sim-telemetry",
  description: "Sim racing telemetry ingestion and analysis"
};

export default function RootLayout({ children }: { children: ReactNode }) {
  return (
    <html lang="en">
      <body>
        <header className="site-header">
          <Link href="/">sim-telemetry</Link>
          <nav>
            <Link href="/sessions">Sessions</Link>
          </nav>
        </header>
        <main>{children}</main>
      </body>
    </html>
  );
}
