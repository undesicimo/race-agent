import { getAuth } from "../../../../lib/auth";

export async function POST(request: Request) {
  const body: unknown = await request.json().catch(() => ({}));
  const requestedName =
    typeof body === "object" && body !== null && "name" in body ? body.name : undefined;
  const name =
    typeof requestedName === "string" && requestedName.trim() ? requestedName.trim() : "Collector";

  const apiKey = await getAuth().api.createApiKey({
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
