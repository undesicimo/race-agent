import { HeartbeatSchema } from "@race-agent/telemetry-schema";
import { insertCollectorHeartbeat } from "@race-agent/database";
import { NextResponse } from "next/server";

import { getDb } from "../../../../lib/db";

export async function POST(request: Request) {
  const payload = HeartbeatSchema.safeParse(await request.json());
  if (!payload.success) {
    return NextResponse.json({ error: payload.error.flatten() }, { status: 400 });
  }

  const receivedAt = await insertCollectorHeartbeat(getDb(), payload.data);

  return NextResponse.json({ ok: true, receivedAt: receivedAt.toISOString() });
}
