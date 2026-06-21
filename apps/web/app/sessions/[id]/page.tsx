import {
  cars,
  collectorHeartbeats,
  collectors,
  sessions,
  telemetrySamples,
  tracks
} from "@race-agent/database";
import { desc, eq, sql } from "drizzle-orm";
import Link from "next/link";
import { notFound } from "next/navigation";

import { getDb } from "../../../lib/db";

export const dynamic = "force-dynamic";

const CONNECTION_WINDOW_MS = 60_000;

export default async function SessionDetailPage({
  params
}: {
  params: Promise<{ id: string }>;
}) {
  const { id } = await params;
  const db = getDb();

  const sessionRows = await db
    .select({
      id: sessions.id,
      sim: sessions.sim,
      startedAt: sessions.startedAt,
      endedAt: sessions.endedAt,
      collectorId: sessions.collectorId,
      collectorName: collectors.name,
      carName: cars.name,
      trackName: tracks.name
    })
    .from(sessions)
    .leftJoin(collectors, eq(sessions.collectorId, collectors.id))
    .leftJoin(cars, eq(sessions.carId, cars.id))
    .leftJoin(tracks, eq(sessions.trackId, tracks.id))
    .where(eq(sessions.id, id))
    .limit(1);

  if (sessionRows.length === 0) {
    notFound();
  }

  const session = sessionRows[0];
  const [heartbeatRows, sampleCountRows, sampleRows] = await Promise.all([
    loadLatestHeartbeat(db, session.collectorId),
    db
      .select({ count: sql<number>`count(*)::int` })
      .from(telemetrySamples)
      .where(eq(telemetrySamples.sessionId, session.id)),
    db
      .select({
        recordedAt: telemetrySamples.recordedAt,
        speedKph: telemetrySamples.speedKph,
        rpm: telemetrySamples.rpm,
        gear: telemetrySamples.gear,
        throttle: telemetrySamples.throttle,
        brake: telemetrySamples.brake,
        lapTimeMs: telemetrySamples.lapTimeMs
      })
      .from(telemetrySamples)
      .where(eq(telemetrySamples.sessionId, session.id))
      .orderBy(desc(telemetrySamples.recordedAt))
      .limit(12)
  ]);

  const latestHeartbeat = heartbeatRows.length > 0 ? heartbeatRows[0] : null;
  const isConnected =
    latestHeartbeat !== null &&
    Date.now() - latestHeartbeat.recordedAt.getTime() <= CONNECTION_WINDOW_MS;
  const sampleCount = sampleCountRows[0]?.count ?? 0;

  return (
    <div className="stack">
      <section className="hero">
        <Link className="back-link" href="/sessions">
          Back to sessions
        </Link>
        <h1>{session.trackName ?? "Session"}</h1>
        <p className="hero-sub">
          {session.carName ?? "Unknown car"} / {session.sim.toUpperCase()} /{" "}
          {session.endedAt ? "Ended" : "Live"}
        </p>
      </section>

      <section className="metric-grid">
        <div className="metric">
          <span className="eyebrow">Started</span>
          <strong>{formatDateTime(session.startedAt)}</strong>
        </div>
        <div className="metric">
          <span className="eyebrow">Ended</span>
          <strong>{session.endedAt ? formatDateTime(session.endedAt) : "Live"}</strong>
        </div>
        <div className="metric">
          <span className="eyebrow">Samples</span>
          <strong>{sampleCount.toLocaleString()}</strong>
        </div>
      </section>

      <section className="panel status-panel">
        <div>
          <span className="eyebrow">Collector connection</span>
          <h2>{isConnected ? "Connected" : latestHeartbeat ? "Last seen" : "No connection"}</h2>
        </div>
        {latestHeartbeat ? (
          <div className="connection-details">
            <span className={`status-pill ${isConnected ? "status-good" : "status-muted"}`}>
              {latestHeartbeat.status}
            </span>
            <span>{session.collectorName ?? session.collectorId ?? "Unknown collector"}</span>
            <span>{formatDateTime(latestHeartbeat.recordedAt)}</span>
            {latestHeartbeat.message ? <span>{latestHeartbeat.message}</span> : null}
          </div>
        ) : (
          <p>No heartbeat has been recorded for this collector.</p>
        )}
      </section>

      <section className="panel">
        <h2>Recent samples</h2>
        {sampleRows.length > 0 ? (
          <div className="table-wrap">
            <table>
              <thead>
                <tr>
                  <th>Recorded</th>
                  <th>Speed</th>
                  <th>RPM</th>
                  <th>Gear</th>
                  <th>Throttle</th>
                  <th>Brake</th>
                  <th>Lap time</th>
                </tr>
              </thead>
              <tbody>
                {sampleRows.map((sample) => (
                  <tr key={sample.recordedAt.toISOString()}>
                    <td>{formatTime(sample.recordedAt)}</td>
                    <td>{formatNumber(sample.speedKph, " kph")}</td>
                    <td>{formatNumber(sample.rpm)}</td>
                    <td>{sample.gear ?? "--"}</td>
                    <td>{formatPercent(sample.throttle)}</td>
                    <td>{formatPercent(sample.brake)}</td>
                    <td>{formatLapTime(sample.lapTimeMs)}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        ) : (
          <p>No samples have been ingested for this session yet.</p>
        )}
      </section>
    </div>
  );
}

function loadLatestHeartbeat(db: ReturnType<typeof getDb>, collectorId: string | null) {
  const columns = {
    status: collectorHeartbeats.status,
    message: collectorHeartbeats.message,
    recordedAt: collectorHeartbeats.recordedAt
  };

  if (collectorId) {
    return db
      .select(columns)
      .from(collectorHeartbeats)
      .where(eq(collectorHeartbeats.collectorId, collectorId))
      .orderBy(desc(collectorHeartbeats.recordedAt))
      .limit(1);
  }

  return db
    .select(columns)
    .from(collectorHeartbeats)
    .orderBy(desc(collectorHeartbeats.recordedAt))
    .limit(1);
}

function formatDateTime(value: Date) {
  return new Intl.DateTimeFormat("en", {
    dateStyle: "medium",
    timeStyle: "medium"
  }).format(value);
}

function formatTime(value: Date) {
  return new Intl.DateTimeFormat("en", {
    hour: "numeric",
    minute: "2-digit",
    second: "2-digit"
  }).format(value);
}

function formatNumber(value: number | null, suffix = "") {
  return typeof value === "number" ? `${Math.round(value).toLocaleString()}${suffix}` : "--";
}

function formatPercent(value: number | null) {
  return typeof value === "number" ? `${Math.round(value * 100)}%` : "--";
}

function formatLapTime(value: number | null) {
  if (typeof value !== "number") {
    return "--";
  }

  const minutes = Math.floor(value / 60_000);
  const seconds = Math.floor((value % 60_000) / 1000);
  const milliseconds = value % 1000;

  return `${minutes}:${seconds.toString().padStart(2, "0")}.${milliseconds
    .toString()
    .padStart(3, "0")}`;
}
