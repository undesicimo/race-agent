# Database

Postgres is the initial database. Drizzle owns schema definitions and migrations.

Set `DATABASE_URL` before running Drizzle commands.

```sh
pnpm --filter @sim-telemetry/database db:generate
pnpm --filter @sim-telemetry/database db:migrate
```

Core tables:

- `collectors`
- `sessions`
- `laps`
- `telemetry_samples`
- `session_events`
- `cars`
- `tracks`

V1 stores useful normalized fields first. Raw/debug capture should be optional and disabled by default.

Simulator-specific adapters should not require new tables for normal telemetry. Add simulator IDs and optional metadata only when the normalized model cannot represent useful common data.

Sampling guidance:

- Live display: 10-30 Hz.
- Stored samples: 10-20 Hz.
- Events: on change/completion.
- Session summary: once per session.

TimescaleDB can be evaluated later if raw sample volume becomes a bottleneck.
