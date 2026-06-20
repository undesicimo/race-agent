# sim-telemetry

OSS monorepo for sim racing telemetry collection, ingestion, visualization, and AI-assisted analysis.

## Milestone 1

- ACC collector runs on Windows.
- Collector reads speed, RPM, gear, throttle, brake, and lap time.
- Collector uploads telemetry batches to the Next.js ingest API.
- Postgres stores sessions and normalized telemetry samples.
- Web app shows one live/current session page.

## Workspaces

- `apps/collector-windows`: Rust console collector.
- `apps/web`: Next.js app for ingest and visualization.
- `apps/mcp`: MCP server for AI analysis tools.
- `crates/telemetry-core`: shared Rust telemetry types.
- `crates/acc-shared-memory`: ACC shared-memory reader.
- `crates/collector-core`: collector adapter/upload logic.
- `packages/telemetry-schema`: Zod schemas and TypeScript telemetry types.
- `packages/database`: Drizzle schema and database access.
- `packages/analysis`: reusable lap and session analysis logic.

## Collector Extension Model

The collector is open for new simulator adapters and closed for changes to upload, ingest, storage, and analysis. New games should map their data source into the shared `TelemetryFrame` type through a `SimAdapter` implementation.

See `docs/sim-adapters.md`.

## Development

```sh
pnpm install
cargo check --workspace
pnpm typecheck
```
