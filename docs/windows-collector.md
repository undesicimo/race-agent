# Windows Collector

The Windows collector should ship as a normal `.exe` that lives in the system tray and only opens a tiny settings window when needed.

## Supported Windows Versions

The supported client floor is Windows 10 x64. Windows 11 x64 is supported.
Older Windows versions are not targeted.

## UX Shape

On launch:

- Start in the tray.
- Auto-start collecting if a saved config already has `server` and `token`.
- Open the settings window automatically on first run or when required config is missing.

Minimal UI:

- One small window with:
  - server URL
  - ingest token
  - fixed simulator label (`ACC` for now)
  - current collector status
  - `Save`, `Start`, and `Stop` actions
- No large dashboard inside the collector itself.
- Closing the window should hide it back to the tray instead of exiting the process.

Tray menu:

- `Open`
- `Start`
- `Stop`
- `Quit`

## Runtime Responsibilities

The tray executable still uses the same collector core responsibilities:

- Select and run a simulator adapter.
- Receive normalized telemetry frames from that adapter.
- Batch frames according to collector policy.
- Upload batches to the Next.js ingest API.
- Persist local collector config for relaunch.

The Windows collector should not contain simulator-specific parsing logic directly. That belongs in adapter crates.

## Runtime Model

The collector should be split into two layers:

1. `collector_service`
   - headless background runtime
   - owns connection retry, session lifecycle, batching, uploads, and shutdown
   - emits status events for UI and tray state
2. Windows shell
   - native tray icon
   - tiny config/status window
   - starts and stops the background service
   - stores config under the user profile

This keeps the tray shell replaceable without rewriting collector logic.

## Current Adapter

The first adapter is ACC:

- Connects to ACC shared memory.
- Reads stable frames using packet ID double-read safety.
- Normalizes frames into shared telemetry fields.

## Packaging Target

The Windows app should build to a single tray-first executable:

```sh
cargo build -p collector-windows --release
```

For local debugging on Windows, the binary can still accept launch overrides:

```sh
collector-windows.exe --server http://server.local:3000 --token xxx --show
```
