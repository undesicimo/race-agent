import { SessionEndSchema } from "@race-agent/telemetry-schema";
import { endIngestSession } from "@race-agent/database";
import { NextResponse } from "next/server";

import { getDb } from "../../../../../lib/db";
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

  const { session, endedAt } = await endIngestSession(getDb(), payload.data);

  if (session === undefined) {
    return NextResponse.json({ error: "Session not found" }, { status: 404 });
  }

  return NextResponse.json({
    ok: true,
    sessionId: session.id,
    endedAt: session.endedAt?.toISOString() ?? endedAt.toISOString()
  });
}
