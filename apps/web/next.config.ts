import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  transpilePackages: [
    "@race-agent/analysis",
    "@race-agent/database",
    "@race-agent/telemetry-schema"
  ]
};

export default nextConfig;
