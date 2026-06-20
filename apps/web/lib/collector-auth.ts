import { auth } from "./auth";

export async function verifyCollectorRequest(request: Request) {
  const header = request.headers.get("authorization");
  const token = header?.match(/^Bearer\s+(.+)$/i)?.[1]?.trim();

  if (!token) {
    return {
      ok: false as const,
      response: Response.json({ error: "Missing bearer token" }, { status: 401 })
    };
  }

  const result = await auth.api.verifyApiKey({
    body: {
      key: token,
      permissions: {
        telemetry: ["write"]
      }
    }
  });

  if (!result.valid || !result.key) {
    return {
      ok: false as const,
      response: Response.json(
        { error: result.error?.message ?? "Invalid bearer token" },
        { status: 401 }
      )
    };
  }

  return {
    ok: true as const,
    apiKey: result.key
  };
}
