# Development

## Prerequisites

- Node.js and pnpm.
- Rust stable toolchain.
- Docker Desktop or another Docker Compose runtime for local Postgres/TimescaleDB.

## Run Locally

Start the database, run migrations, and start the web app:

```sh
cp .env.example .env.local
pnpm dev
```

The web app runs on the port selected by Next.js, usually `http://localhost:3000`.
Docker must be running before `pnpm db:up`, `pnpm dev:local`, or `pnpm dev`.

To run the steps manually:

```sh
pnpm db:up
pnpm db:migrate
pnpm dev:web
```

Useful database commands:

```sh
pnpm db:logs
pnpm db:stop
```

The default local database URL is:

```txt
postgres://postgres:postgres@localhost:5432/race_agent
```

## Commands

```sh
pnpm install
pnpm typecheck
cargo check --workspace
pnpm dev
pnpm dev:web
pnpm check
```

## Layout

```txt
apps/
  collector-windows/
  web/
  mcp/
crates/
  telemetry-core/
  acc-shared-memory/
  collector-core/
packages/
  telemetry-schema/
  database/
  analysis/
docs/
```

Start with `docs/architecture.md` and `docs/sim-adapters.md` before adding simulator-specific code.
