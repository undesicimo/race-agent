# API

The collector uploads telemetry to the Next.js app. All request bodies are validated with Zod schemas from `packages/telemetry-schema`.

The API accepts normalized telemetry, not simulator-native packets. The `sim` value identifies the source adapter, but the payload shape should remain stable across supported games.

## Endpoints

```txt
POST /api/ingest/heartbeat
POST /api/ingest/session/start
POST /api/ingest/telemetry/batch
POST /api/ingest/session/end
```

## Telemetry Batch

```json
{
  "sim": "acc",
  "sessionId": "00000000-0000-0000-0000-000000000000",
  "samples": [
    {
      "timestamp": "2026-06-20T00:00:00.000Z",
      "speedKph": 123.4,
      "rpm": 7200,
      "gear": 4,
      "throttle": 1,
      "brake": 0,
      "lapTimeMs": 93210
    }
  ]
}
```
