import {
  boolean,
  doublePrecision,
  index,
  integer,
  jsonb,
  primaryKey,
  pgEnum,
  pgTable,
  text,
  timestamp,
  uniqueIndex,
  uuid
} from "drizzle-orm/pg-core";

export const simEnum = pgEnum("sim", ["acc", "iracing", "assetto_corsa", "generic"]);

export const apikey = pgTable(
  "apikey",
  {
    id: text("id").primaryKey(),
    configId: text("config_id").notNull().default("default"),
    name: text("name"),
    start: text("start"),
    prefix: text("prefix"),
    key: text("key").notNull(),
    referenceId: text("reference_id").notNull(),
    refillInterval: integer("refill_interval"),
    refillAmount: integer("refill_amount"),
    lastRefillAt: timestamp("last_refill_at", { withTimezone: true }),
    enabled: boolean("enabled"),
    rateLimitEnabled: boolean("rate_limit_enabled"),
    rateLimitTimeWindow: integer("rate_limit_time_window"),
    rateLimitMax: integer("rate_limit_max"),
    requestCount: integer("request_count"),
    remaining: integer("remaining"),
    lastRequest: timestamp("last_request", { withTimezone: true }),
    expiresAt: timestamp("expires_at", { withTimezone: true }),
    createdAt: timestamp("created_at", { withTimezone: true }).notNull(),
    updatedAt: timestamp("updated_at", { withTimezone: true }).notNull(),
    permissions: text("permissions"),
    metadata: text("metadata")
  },
  (table) => [
    index("apikey_config_id_idx").on(table.configId),
    index("apikey_reference_id_idx").on(table.referenceId)
  ]
);

export const collectors = pgTable("collectors", {
  id: uuid("id").primaryKey().defaultRandom(),
  name: text("name"),
  tokenHash: text("token_hash"),
  createdAt: timestamp("created_at", { withTimezone: true }).notNull().defaultNow()
});

export const cars = pgTable(
  "cars",
  {
    id: uuid("id").primaryKey().defaultRandom(),
    sim: simEnum("sim").notNull(),
    name: text("name").notNull()
  },
  (table) => [uniqueIndex("cars_sim_name_idx").on(table.sim, table.name)]
);

export const tracks = pgTable(
  "tracks",
  {
    id: uuid("id").primaryKey().defaultRandom(),
    sim: simEnum("sim").notNull(),
    name: text("name").notNull()
  },
  (table) => [uniqueIndex("tracks_sim_name_idx").on(table.sim, table.name)]
);

export const sessions = pgTable("sessions", {
  id: uuid("id").primaryKey().defaultRandom(),
  collectorId: uuid("collector_id").references(() => collectors.id),
  sim: simEnum("sim").notNull(),
  carId: uuid("car_id").references(() => cars.id),
  trackId: uuid("track_id").references(() => tracks.id),
  startedAt: timestamp("started_at", { withTimezone: true }).notNull().defaultNow(),
  endedAt: timestamp("ended_at", { withTimezone: true })
});

export const laps = pgTable("laps", {
  id: uuid("id").primaryKey().defaultRandom(),
  sessionId: uuid("session_id").notNull().references(() => sessions.id),
  lapNumber: integer("lap_number").notNull(),
  lapTimeMs: integer("lap_time_ms"),
  createdAt: timestamp("created_at", { withTimezone: true }).notNull().defaultNow()
});

export const telemetrySamples = pgTable(
  "telemetry_samples",
  {
    id: uuid("id").defaultRandom().notNull(),
    sessionId: uuid("session_id"),
    lapId: uuid("lap_id").references(() => laps.id),
    recordedAt: timestamp("recorded_at", { withTimezone: true }).notNull(),
    speedKph: doublePrecision("speed_kph"),
    rpm: integer("rpm"),
    gear: integer("gear"),
    throttle: doublePrecision("throttle"),
    brake: doublePrecision("brake"),
    clutch: doublePrecision("clutch"),
    steering: doublePrecision("steering"),
    lapTimeMs: integer("lap_time_ms"),
    normalizedTrackPosition: doublePrecision("normalized_track_position"),
    fuelLiters: doublePrecision("fuel_liters"),
    tyres: jsonb("tyres")
  },
  (table) => [
    primaryKey({ columns: [table.id, table.recordedAt] }),
    index("telemetry_samples_session_recorded_at_idx").on(table.sessionId, table.recordedAt),
    index("telemetry_samples_recorded_at_idx").on(table.recordedAt)
  ]
);

export const sessionEvents = pgTable("session_events", {
  id: uuid("id").primaryKey().defaultRandom(),
  sessionId: uuid("session_id").notNull().references(() => sessions.id),
  type: text("type").notNull(),
  payload: jsonb("payload"),
  recordedAt: timestamp("recorded_at", { withTimezone: true }).notNull().defaultNow()
});

export const collectorHeartbeats = pgTable(
  "collector_heartbeats",
  {
    id: uuid("id").primaryKey().defaultRandom(),
    collectorId: uuid("collector_id").references(() => collectors.id),
    sim: simEnum("sim").notNull(),
    status: text("status").notNull(),
    message: text("message"),
    recordedAt: timestamp("recorded_at", { withTimezone: true }).notNull().defaultNow()
  },
  (table) => [
    index("collector_heartbeats_collector_recorded_at_idx").on(
      table.collectorId,
      table.recordedAt
    )
  ]
);
