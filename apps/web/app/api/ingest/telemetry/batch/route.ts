import { TelemetryBatchSchema } from "@sim-telemetry/telemetry-schema";
import { insertTelemetryBatch } from "@sim-telemetry/database";
import { NextResponse } from "next/server";

import { db } from "../../../../../lib/db";
import { verifyCollectorRequest } from "../../../../../lib/collector-auth";

export async function POST(request: Request) {
  const collector = await verifyCollectorRequest(request);
  if (!collector.ok) {
    return collector.response;
  }

  const payload = TelemetryBatchSchema.safeParse(await request.json());
  if (!payload.success) {
    return NextResponse.json({ error: payload.error.flatten() }, { status: 400 });
  }

  await insertTelemetryBatch(db, payload.data);

  return NextResponse.json({
    ok: true,
    sessionId: payload.data.sessionId,
    acceptedSamples: payload.data.samples.length
  });
}
