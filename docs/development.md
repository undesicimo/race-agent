# Development

## Prerequisites

- Node.js and pnpm.
- Rust stable toolchain.
- Postgres for persistence once ingest storage is implemented.

## Commands

```sh
pnpm install
pnpm typecheck
cargo check --workspace
pnpm dev:web
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
