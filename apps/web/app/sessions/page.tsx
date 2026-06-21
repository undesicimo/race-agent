import { cars, collectorHeartbeats, collectors, sessions, tracks } from "@race-agent/database";
import { desc, eq } from "drizzle-orm";
import Link from "next/link";

import { getDb } from "../../lib/db";

export const dynamic = "force-dynamic";

const CONNECTION_WINDOW_MS = 60_000;

export default async function SessionsPage() {
  const db = getDb();
  const [sessionRows, heartbeatRows] = await Promise.all([
    db
      .select({
        id: sessions.id,
        sim: sessions.sim,
        startedAt: sessions.startedAt,
        endedAt: sessions.endedAt,
        carName: cars.name,
        trackName: tracks.name,
        collectorName: collectors.name,
        collectorId: sessions.collectorId
      })
      .from(sessions)
      .leftJoin(cars, eq(sessions.carId, cars.id))
      .leftJoin(tracks, eq(sessions.trackId, tracks.id))
      .leftJoin(collectors, eq(sessions.collectorId, collectors.id))
      .orderBy(desc(sessions.startedAt))
      .limit(20),
    db
      .select({
        sim: collectorHeartbeats.sim,
        status: collectorHeartbeats.status,
        message: collectorHeartbeats.message,
        recordedAt: collectorHeartbeats.recordedAt,
        collectorName: collectors.name,
        collectorId: collectorHeartbeats.collectorId
      })
      .from(collectorHeartbeats)
      .leftJoin(collectors, eq(collectorHeartbeats.collectorId, collectors.id))
      .orderBy(desc(collectorHeartbeats.recordedAt))
      .limit(1)
  ]);

  const latestHeartbeat = heartbeatRows.length > 0 ? heartbeatRows[0] : null;
  const isConnected =
    latestHeartbeat !== null &&
    Date.now() - latestHeartbeat.recordedAt.getTime() <= CONNECTION_WINDOW_MS;

  return (
    <div className="stack">
      <section className="hero">
        <h1>Sessions</h1>
        <p className="hero-sub">
          Recent telemetry sessions and collector connection state.
        </p>
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
            <span>
              {latestHeartbeat.collectorName ?? latestHeartbeat.collectorId ?? "Unknown collector"}{" "}
              on {latestHeartbeat.sim.toUpperCase()}
            </span>
            <span>{formatDateTime(latestHeartbeat.recordedAt)}</span>
            {latestHeartbeat.message ? <span>{latestHeartbeat.message}</span> : null}
          </div>
        ) : (
          <p>No collector heartbeat has reached this server yet.</p>
        )}
      </section>

      <section className="panel">
        <h2>Recent sessions</h2>
        {sessionRows.length > 0 ? (
          <div className="session-list">
            {sessionRows.map((session) => (
              <Link key={session.id} className="session-row" href={`/sessions/${session.id}`}>
                <div>
                  <strong>{session.trackName ?? "Unknown track"}</strong>
                  <span>
                    {session.carName ?? "Unknown car"} / {session.sim.toUpperCase()}
                  </span>
                </div>
                <div>
                  <span
                    className={`status-pill ${session.endedAt ? "status-muted" : "status-good"}`}
                  >
                    {session.endedAt ? "Ended" : "Live"}
                  </span>
                  <span>{formatDateTime(session.startedAt)}</span>
                </div>
                <div>
                  <span className="eyebrow">Collector</span>
                  <span>{session.collectorName ?? session.collectorId ?? "Unknown"}</span>
                </div>
              </Link>
            ))}
          </div>
        ) : (
          <p>No sessions yet. Connect a collector to begin ingesting telemetry.</p>
        )}
      </section>
    </div>
  );
}

function formatDateTime(value: Date) {
  return new Intl.DateTimeFormat("en", {
    dateStyle: "medium",
    timeStyle: "medium"
  }).format(value);
}
