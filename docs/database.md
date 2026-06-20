# Database

Postgres with TimescaleDB is the initial database. Drizzle owns schema definitions and migrations.

## Local Database

Start TimescaleDB locally with Docker Compose:

```sh
pnpm db:up
```

Run migrations:

```sh
pnpm db:migrate
```

Stop the local database:

```sh
pnpm db:stop
```

Tail database logs:

```sh
pnpm db:logs
```

Set `DATABASE_URL` before running Drizzle commands.

```sh
export DATABASE_URL=postgres://postgres:postgres@localhost:5432/sim_telemetry
pnpm db:generate
pnpm db:migrate
```

Core tables:

- `collectors`
- `sessions`
- `laps`
- `telemetry_samples`
- `session_events`
- `cars`
- `tracks`
- `apikey`
- `collector_heartbeats`

V1 stores useful normalized fields first. Raw/debug capture should be optional and disabled by default.

Simulator-specific adapters should not require new tables for normal telemetry. Add simulator IDs and optional metadata only when the normalized model cannot represent useful common data.

Sampling guidance:

- Live display: 10-30 Hz.
- Stored samples: 10-20 Hz.
- Events: on change/completion.
- Session summary: once per session.

`telemetry_samples` is a TimescaleDB hypertable partitioned by `recorded_at`.
