import { and, eq } from "drizzle-orm";

import type {
  Heartbeat,
  SessionEnd,
  SessionStart,
  TelemetryBatch
} from "@race-agent/telemetry-schema";
import type { PostgresJsDatabase } from "drizzle-orm/postgres-js";

import {
  cars,
  collectorHeartbeats,
  sessions,
  telemetrySamples,
  tracks
} from "./schema";
import type * as schema from "./schema";

type Database = PostgresJsDatabase<typeof schema>;
type Sim = SessionStart["sim"];

async function findOrCreateCar(db: Database, sim: Sim, name?: string) {
  if (!name) {
    return null;
  }

  const existingRows = await db
    .select({ id: cars.id })
    .from(cars)
    .where(and(eq(cars.sim, sim), eq(cars.name, name)))
    .limit(1);

  if (existingRows.length > 0) {
    return existingRows[0].id;
  }

  const [created] = await db.insert(cars).values({ sim, name }).returning({ id: cars.id });
  return created.id;
}

async function findOrCreateTrack(db: Database, sim: Sim, name?: string) {
  if (!name) {
    return null;
  }

  const existingRows = await db
    .select({ id: tracks.id })
    .from(tracks)
    .where(and(eq(tracks.sim, sim), eq(tracks.name, name)))
    .limit(1);

  if (existingRows.length > 0) {
    return existingRows[0].id;
  }

  const [created] = await db.insert(tracks).values({ sim, name }).returning({ id: tracks.id });
  return created.id;
}

export async function startIngestSession(db: Database, payload: SessionStart) {
  const [carId, trackId] = await Promise.all([
    findOrCreateCar(db, payload.sim, payload.carName),
    findOrCreateTrack(db, payload.sim, payload.trackName)
  ]);

  const [session] = await db
    .insert(sessions)
    .values({
      collectorId: payload.collectorId,
      sim: payload.sim,
      carId,
      trackId,
      startedAt: payload.startedAt ? new Date(payload.startedAt) : new Date()
    })
    .returning({ id: sessions.id, startedAt: sessions.startedAt });

  return session;
}

export async function endIngestSession(db: Database, payload: SessionEnd) {
  const endedAt = payload.endedAt ? new Date(payload.endedAt) : new Date();
  const sessionRows = await db
    .update(sessions)
    .set({ endedAt })
    .where(eq(sessions.id, payload.sessionId))
    .returning({ id: sessions.id, endedAt: sessions.endedAt });

  return {
    session: sessionRows.length > 0 ? sessionRows[0] : undefined,
    endedAt
  };
}

export async function insertTelemetryBatch(db: Database, payload: TelemetryBatch) {
  await db.insert(telemetrySamples).values(
    payload.samples.map((sample) => ({
      sessionId: payload.sessionId ?? null,
      recordedAt: new Date(sample.timestamp),
      speedKph: sample.speedKph,
      rpm: sample.rpm,
      gear: sample.gear,
      throttle: sample.throttle,
      brake: sample.brake,
      clutch: sample.clutch,
      steering: sample.steering,
      lapTimeMs: sample.lapTimeMs,
      normalizedTrackPosition: sample.normalizedTrackPosition,
      fuelLiters: sample.fuelLiters,
      tyres: sample.tyres
    }))
  );
}

export async function insertCollectorHeartbeat(db: Database, payload: Heartbeat) {
  const recordedAt = payload.timestamp ? new Date(payload.timestamp) : new Date();

  await db.insert(collectorHeartbeats).values({
    collectorId: payload.collectorId,
    sim: payload.sim,
    status: payload.status,
    message: payload.message,
    recordedAt
  });

  return recordedAt;
}
