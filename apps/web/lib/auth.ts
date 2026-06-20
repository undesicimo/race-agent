import { apiKey } from "@better-auth/api-key";
import { betterAuth } from "better-auth";
import { drizzleAdapter } from "better-auth/adapters/drizzle";

import { db } from "./db";

export const auth = betterAuth({
  database: drizzleAdapter(db, {
    provider: "pg"
  }),
  plugins: [
    apiKey({
      defaultPrefix: "race_",
      rateLimit: {
        enabled: false
      },
      permissions: {
        defaultPermissions: {
          telemetry: ["write"]
        }
      }
    })
  ]
});
