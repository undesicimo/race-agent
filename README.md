# sim-telemetry

OSS monorepo for sim racing telemetry collection, ingestion, visualization, and AI-assisted analysis.

The first milestone is an end-to-end Assetto Corsa Competizione flow:

1. A Windows collector reads ACC shared memory.
2. The collector normalizes frames into common telemetry payloads.
3. The Next.js app authenticates collector requests and stores sessions/samples in Postgres + TimescaleDB.
4. The web app shows recorded and live telemetry sessions.

## Quick Start

Prerequisites:

- Node.js
- pnpm
- Rust stable
- Docker Desktop or another Docker Compose runtime

Install dependencies:

```sh
pnpm install
```

Start the full local stack:

```sh
cp .env.example .env.local
pnpm dev
```

`pnpm dev` starts TimescaleDB with Docker Compose, waits for the database healthcheck, runs Drizzle migrations, and starts the Next.js web app.

The web app usually runs at:

```txt
http://localhost:3000
```

Docker must be running before `pnpm dev`.

## Useful Commands

```sh
pnpm dev          # Start database, run migrations, start web app
pnpm dev:web      # Start only the Next.js app
pnpm db:up        # Start TimescaleDB/Postgres
pnpm db:migrate   # Run Drizzle migrations
pnpm db:logs      # Tail database logs
pnpm db:stop      # Stop the database container
pnpm check        # Run lint, TypeScript checks, and Rust checks
pnpm lint         # Run Oxlint
pnpm typecheck    # Run TypeScript checks
pnpm rust:check   # Run cargo check --workspace
```

## Local Database

The default local database URL is:

```txt
postgres://postgres:postgres@localhost:5432/sim_telemetry
```

It is defined in [.env.example](./.env.example). Copy that file to `.env.local` before running the app.

Database schema and migrations live in `packages/database`.

## Workspaces

- `apps/web`: Next.js app for ingest, auth, and visualization.
- `apps/mcp`: MCP server for AI analysis tools.
- `apps/collector-windows`: Rust Windows collector.
- `crates/telemetry-core`: shared Rust telemetry types.
- `crates/acc-shared-memory`: ACC shared-memory reader.
- `crates/collector-core`: collector adapter/upload logic.
- `packages/telemetry-schema`: Zod schemas and TypeScript telemetry types.
- `packages/database`: Drizzle schema, migrations, and database access.
- `packages/analysis`: reusable lap and session analysis logic.

## Collector Extension Model

The collector is open for new simulator adapters and closed for changes to upload, ingest, storage, and analysis. New games should map their data source into the shared `TelemetryFrame` type through a `SimAdapter` implementation.

See [docs/sim-adapters.md](./docs/sim-adapters.md).

## More Docs

- [Architecture](./docs/architecture.md)
- [API](./docs/api.md)
- [Database](./docs/database.md)
- [Development](./docs/development.md)
- [Windows collector](./docs/windows-collector.md)
