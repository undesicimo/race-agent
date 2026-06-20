import { HeartbeatSchema } from "@sim-telemetry/telemetry-schema";
import { insertCollectorHeartbeat } from "@sim-telemetry/database";
import { NextResponse } from "next/server";

import { db } from "../../../../lib/db";
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

  const receivedAt = await insertCollectorHeartbeat(db, payload.data);

  return NextResponse.json({ ok: true, receivedAt: receivedAt.toISOString() });
}
