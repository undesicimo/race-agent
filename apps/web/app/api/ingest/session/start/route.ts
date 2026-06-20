import { SessionStartSchema } from "@sim-telemetry/telemetry-schema";
import { NextResponse } from "next/server";

export async function POST(request: Request) {
  const payload = SessionStartSchema.safeParse(await request.json());
  if (!payload.success) {
    return NextResponse.json({ error: payload.error.flatten() }, { status: 400 });
  }

  return NextResponse.json({
    ok: true,
    sessionId: crypto.randomUUID(),
    startedAt: payload.data.startedAt ?? new Date().toISOString()
  });
}
