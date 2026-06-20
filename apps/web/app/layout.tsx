import type { Metadata } from "next";
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
          <a href="/">sim-telemetry</a>
          <nav>
            <a href="/sessions">Sessions</a>
          </nav>
        </header>
        <main>{children}</main>
      </body>
    </html>
  );
}
