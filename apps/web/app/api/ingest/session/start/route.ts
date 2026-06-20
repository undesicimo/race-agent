import { SessionStartSchema } from "@sim-telemetry/telemetry-schema";
import { startIngestSession } from "@sim-telemetry/database";
import { NextResponse } from "next/server";

import { getDb } from "../../../../../lib/db";
import { verifyCollectorRequest } from "../../../../../lib/collector-auth";

export async function POST(request: Request) {
  const collector = await verifyCollectorRequest(request);
  if (!collector.ok) {
    return collector.response;
  }

  const payload = SessionStartSchema.safeParse(await request.json());
  if (!payload.success) {
    return NextResponse.json({ error: payload.error.flatten() }, { status: 400 });
  }

  const session = await startIngestSession(getDb(), payload.data);

  return NextResponse.json({
    ok: true,
    sessionId: session.id,
    startedAt: session.startedAt.toISOString()
  });
}
