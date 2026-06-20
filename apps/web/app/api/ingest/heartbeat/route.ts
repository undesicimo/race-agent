import { HeartbeatSchema } from "@sim-telemetry/telemetry-schema";
import { NextResponse } from "next/server";

export async function POST(request: Request) {
  const payload = HeartbeatSchema.safeParse(await request.json());
  if (!payload.success) {
    return NextResponse.json({ error: payload.error.flatten() }, { status: 400 });
  }

  return NextResponse.json({ ok: true, receivedAt: new Date().toISOString() });
}
