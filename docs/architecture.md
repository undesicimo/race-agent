# Architecture

`race-agent` is split into three runtime surfaces:

- Windows collector: reads local simulator telemetry and uploads normalized batches.
- Next.js web app: owns ingestion, storage, and visualization.
- MCP server: lets AI agents query stored telemetry and reuse shared analysis logic.

The collector never talks directly to Postgres. It sends validated API payloads to the web app, and the web app persists normalized records.

## Open/Closed Collector Design

The collector should be open for new simulator adapters and closed for changes to upload, ingestion, storage, and analysis.

Simulator-specific code belongs in adapter crates such as `acc-shared-memory`, `iracing-sdk`, or `generic-udp`. Those adapters normalize data into `TelemetryFrame`. After that point, the rest of the system should not care which simulator produced the sample.

## Data Flow

```txt
sim data source
  -> simulator adapter crate
  -> TelemetryFrame
  -> collector-core
  -> POST /api/ingest/telemetry/batch
  -> packages/telemetry-schema validation
  -> packages/database
  -> Postgres
  -> web visualizations and MCP tools
```

## Package Boundaries

- `crates/telemetry-core` defines normalized Rust telemetry types used by collectors.
- `crates/collector-core` owns simulator-independent batching and upload logic.
- `crates/*-shared-memory`, `crates/*-sdk`, and protocol crates own simulator-specific reads and normalization.
- `packages/telemetry-schema` defines API payload schemas with Zod.
- `packages/database` defines the Drizzle schema and database client.
- `packages/analysis` contains reusable racing analysis algorithms.
