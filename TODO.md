# TODO

## Current State

- [x] Monorepo scaffolded with pnpm and Cargo workspaces.
- [x] Next.js web app scaffolded.
- [x] Rust collector crates scaffolded.
- [x] Shared TypeScript packages scaffolded.
- [x] Initial docs and OSS license added.
- [x] Simulator adapter boundary documented.
- [x] ACC shared-memory structs and Windows mapping access implemented.
- [x] ACC packet ID double-read safety added.
- [x] ACC frames normalized into `TelemetryFrame`.
- [x] Collector CLI upload loop implemented.
- [x] Ingest API routes added.
- [x] Collector API-key authentication added.
- [x] Drizzle schema and migrations added for sessions, samples, auth, and heartbeats.
- [x] Oxlint AI-code-smell harness added with type-aware checks.
- [x] Workspace updated to TypeScript 6.

## Next For You

1. [ ] Stand up local Postgres/TimescaleDB and run migrations.
   - Run `cp .env.example .env.local`.
   - Run `pnpm dev`.
   - Confirm the Timescale hypertable migration applies cleanly.

2. [ ] Smoke-test ingest end to end without the Windows collector.
   - Start the web app with `pnpm dev:web`.
   - Create a collector token through `POST /api/collectors/token`.
   - Call heartbeat, session start, telemetry batch, and session end with the token.
   - Verify rows land in `sessions`, `telemetry_samples`, and `collector_heartbeats`.

3. [ ] Build the first useful sessions page.
   - Replace the placeholder in `apps/web/app/sessions/page.tsx`.
   - List recent sessions from Postgres.
   - Show sim, car, track, start time, end time, and sample count.
   - Link each session to a detail route.

4. [ ] Add a live/current session view.
   - Show the latest active session.
   - Display speed, RPM, gear, throttle, brake, and lap time from recent samples.
   - Refresh with polling first; defer WebSockets until the basic UI works.

5. [ ] Add ingest integration tests.
   - Cover unauthorized requests.
   - Cover invalid Zod payloads.
   - Cover session start, batch insert, and session end against a test database.

## Milestone 1 Exit Criteria

- [ ] Windows ACC collector can create a session against the local web app.
- [ ] Telemetry samples persist in Postgres.
- [ ] `/sessions` shows recorded sessions.
- [ ] A live/current session page shows recent telemetry.
- [ ] `pnpm lint`, `pnpm typecheck`, and `cargo check --workspace` pass.

## Later

- [ ] Lap comparison views.
- [ ] More analysis package algorithms.
- [ ] MCP tools backed by real stored sessions.
- [ ] Windows tray or native UI.
- [ ] iRacing adapter crate.
- [ ] Assetto Corsa adapter crate.
- [ ] Generic UDP/HTTP adapter crate.
