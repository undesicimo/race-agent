import { TelemetryBatchSchema } from "@sim-telemetry/telemetry-schema";
import { NextResponse } from "next/server";

export async function POST(request: Request) {
  const payload = TelemetryBatchSchema.safeParse(await request.json());
  if (!payload.success) {
    return NextResponse.json({ error: payload.error.flatten() }, { status: 400 });
  }

  return NextResponse.json({
    ok: true,
    sessionId: payload.data.sessionId,
    acceptedSamples: payload.data.samples.length
  });
}
