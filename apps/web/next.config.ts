import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  transpilePackages: [
    "@sim-telemetry/analysis",
    "@sim-telemetry/database",
    "@sim-telemetry/telemetry-schema"
  ]
};

export default nextConfig;
