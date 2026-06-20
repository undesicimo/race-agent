import { apiKey } from "@better-auth/api-key";
import { betterAuth } from "better-auth";
import { drizzleAdapter } from "better-auth/adapters/drizzle";

import { getDb } from "./db";

function createAuth() {
  return betterAuth({
    database: drizzleAdapter(getDb(), {
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
}

let auth: ReturnType<typeof createAuth> | undefined;

export function getAuth(): ReturnType<typeof createAuth> {
  auth ??= createAuth();

  return auth;
}
