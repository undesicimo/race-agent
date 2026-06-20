import { createDatabase } from "@race-agent/database";

let db: ReturnType<typeof createDatabase> | undefined;

export function getDb() {
  db ??= createDatabase();
  return db;
}
