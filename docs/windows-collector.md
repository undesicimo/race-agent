# Windows Collector

V1 is a console application:

```sh
collector-windows --server http://server.local:3000 --token xxx --sim acc
```

Responsibilities:

- Select and run a simulator adapter.
- Receive normalized telemetry frames from that adapter.
- Batch frames according to collector policy.
- Upload batches to the Next.js ingest API.

The Windows collector should not contain simulator-specific parsing logic directly. That belongs in adapter crates.

## Current Adapter

The first adapter is ACC:

- Connects to ACC shared memory.
- Reads stable frames using packet ID double-read safety.
- Normalizes frames into shared telemetry fields.

The UI/tray app is a later layer over the same collector core.
