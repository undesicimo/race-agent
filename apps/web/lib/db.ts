import { createDatabase } from "@sim-telemetry/database";

let db: ReturnType<typeof createDatabase> | undefined;

export function getDb() {
  db ??= createDatabase();
  return db;
}
