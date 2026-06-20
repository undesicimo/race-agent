import { SessionEndSchema } from "@sim-telemetry/telemetry-schema";
import { endIngestSession } from "@sim-telemetry/database";
import { NextResponse } from "next/server";

import { db } from "../../../../../lib/db";
import { verifyCollectorRequest } from "../../../../../lib/collector-auth";

export async function POST(request: Request) {
  const collector = await verifyCollectorRequest(request);
  if (!collector.ok) {
    return collector.response;
  }

  const payload = SessionEndSchema.safeParse(await request.json());
  if (!payload.success) {
    return NextResponse.json({ error: payload.error.flatten() }, { status: 400 });
  }

  const { session, endedAt } = await endIngestSession(db, payload.data);

  if (!session) {
    return NextResponse.json({ error: "Session not found" }, { status: 404 });
  }

  return NextResponse.json({
    ok: true,
    sessionId: session.id,
    endedAt: session.endedAt?.toISOString() ?? endedAt.toISOString()
  });
}
