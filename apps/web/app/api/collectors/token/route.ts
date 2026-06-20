import { auth } from "../../../../lib/auth";

export async function POST(request: Request) {
  const body = await request.json().catch(() => ({}));
  const name = typeof body.name === "string" && body.name.trim() ? body.name.trim() : "Collector";

  const apiKey = await auth.api.createApiKey({
    body: {
      name,
      prefix: "race_",
      userId: "personal",
      permissions: {
        telemetry: ["write"]
      }
    }
  });

  return Response.json({
    id: apiKey.id,
    name: apiKey.name,
    token: apiKey.key
  });
}
