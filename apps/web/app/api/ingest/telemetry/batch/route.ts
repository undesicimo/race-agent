import { TelemetryBatchSchema } from "@race-agent/telemetry-schema";
import { insertTelemetryBatch } from "@race-agent/database";
import { NextResponse } from "next/server";

import { getDb } from "../../../../../lib/db";

export async function POST(request: Request) {
  const payload = TelemetryBatchSchema.safeParse(await request.json());
  if (!payload.success) {
    return NextResponse.json({ error: payload.error.flatten() }, { status: 400 });
  }

  await insertTelemetryBatch(getDb(), payload.data);

  return NextResponse.json({
    ok: true,
    sessionId: payload.data.sessionId,
    acceptedSamples: payload.data.samples.length
  });
}
