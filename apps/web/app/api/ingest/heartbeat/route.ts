import { HeartbeatSchema } from "@race-agent/telemetry-schema";
import { insertCollectorHeartbeat } from "@race-agent/database";
import { NextResponse } from "next/server";

import { getDb } from "../../../../lib/db";
import { verifyCollectorRequest } from "../../../../lib/collector-auth";

export async function POST(request: Request) {
  const collector = await verifyCollectorRequest(request);
  if (!collector.ok) {
    return collector.response;
  }

  const payload = HeartbeatSchema.safeParse(await request.json());
  if (!payload.success) {
    return NextResponse.json({ error: payload.error.flatten() }, { status: 400 });
  }

  const receivedAt = await insertCollectorHeartbeat(getDb(), payload.data);

  return NextResponse.json({ ok: true, receivedAt: receivedAt.toISOString() });
}
