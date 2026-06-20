# TODO

## Phase 1: Foundation

- [x] Create monorepo directory structure.
- [x] Add pnpm workspace configuration.
- [x] Add Cargo workspace configuration.
- [x] Scaffold Next.js web app.
- [x] Scaffold Rust collector crates.
- [x] Scaffold shared TypeScript packages.
- [x] Add initial docs and OSS license.
- [x] Document open/closed simulator adapter boundary.

## Milestone 1: End-to-End ACC Telemetry

- [x] Implement ACC shared-memory structs and Windows mapping access.
- [x] Add packet ID double-read safety.
- [x] Normalize ACC frames into `TelemetryFrame`.
- [x] Implement collector CLI upload loop.
- [ ] Implement ingest authentication.
- [ ] Persist sessions and telemetry samples in Postgres.
- [ ] Add live/current session web page.
- [ ] Add basic integration test path for ingest validation.

## Later Phases

- [ ] Lap comparison views.
- [ ] Analysis package algorithms.
- [ ] MCP tools.
- [ ] Windows tray or native UI.
- [ ] iRacing adapter crate.
- [ ] Assetto Corsa adapter crate.
- [ ] Generic UDP/HTTP adapter crate.
