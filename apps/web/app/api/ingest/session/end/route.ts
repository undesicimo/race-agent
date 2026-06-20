import { SessionEndSchema } from "@sim-telemetry/telemetry-schema";
import { NextResponse } from "next/server";

export async function POST(request: Request) {
  const payload = SessionEndSchema.safeParse(await request.json());
  if (!payload.success) {
    return NextResponse.json({ error: payload.error.flatten() }, { status: 400 });
  }

  return NextResponse.json({
    ok: true,
    sessionId: payload.data.sessionId,
    endedAt: payload.data.endedAt ?? new Date().toISOString()
  });
}
