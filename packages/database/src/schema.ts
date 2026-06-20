import {
  doublePrecision,
  integer,
  jsonb,
  pgEnum,
  pgTable,
  text,
  timestamp,
  uuid
} from "drizzle-orm/pg-core";

export const simEnum = pgEnum("sim", ["acc", "iracing", "assetto_corsa", "generic"]);

export const collectors = pgTable("collectors", {
  id: uuid("id").primaryKey().defaultRandom(),
  name: text("name"),
  tokenHash: text("token_hash"),
  createdAt: timestamp("created_at", { withTimezone: true }).notNull().defaultNow()
});

export const cars = pgTable("cars", {
  id: uuid("id").primaryKey().defaultRandom(),
  sim: simEnum("sim").notNull(),
  name: text("name").notNull()
});

export const tracks = pgTable("tracks", {
  id: uuid("id").primaryKey().defaultRandom(),
  sim: simEnum("sim").notNull(),
  name: text("name").notNull()
});

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

export const telemetrySamples = pgTable("telemetry_samples", {
  id: uuid("id").primaryKey().defaultRandom(),
  sessionId: uuid("session_id").notNull().references(() => sessions.id),
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
});

export const sessionEvents = pgTable("session_events", {
  id: uuid("id").primaryKey().defaultRandom(),
  sessionId: uuid("session_id").notNull().references(() => sessions.id),
  type: text("type").notNull(),
  payload: jsonb("payload"),
  recordedAt: timestamp("recorded_at", { withTimezone: true }).notNull().defaultNow()
});
