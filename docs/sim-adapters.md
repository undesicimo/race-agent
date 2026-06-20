# Sim Adapters

The collector is designed around an open/closed boundary:

- Open for new simulator adapters.
- Closed for collector upload, batching, API ingestion, storage, and analysis changes when a new simulator is added.

New racing games should be added by implementing the Rust `SimAdapter` contract and mapping simulator-specific data into `TelemetryFrame`.

## Adapter Contract

Every adapter is responsible for:

- Connecting to the simulator data source.
- Reading stable frames from that source.
- Mapping simulator-specific fields into normalized telemetry fields.
- Returning `None` when no fresh frame is available.
- Avoiding direct network, database, or web API concerns.

The collector core owns:

- Batching.
- Upload retries.
- Authentication headers.
- Server endpoint selection.
- Sample rate policy.

## Rust Shape

```rust
#[async_trait]
pub trait SimAdapter {
    async fn connect(&mut self) -> Result<()>;
    fn sim(&self) -> Sim;
    async fn next_frame(&mut self) -> Result<Option<TelemetryFrame>>;
}
```

## Adding A Simulator

Add one crate per simulator or protocol:

```txt
crates/
  acc-shared-memory/
  iracing-sdk/
  assetto-corsa-shared-memory/
  generic-udp/
```

Each crate should expose an adapter that produces the shared `TelemetryFrame`.

## Normalized Fields

Adapters should fill the common fields when available:

- speed
- RPM
- gear
- throttle
- brake
- clutch
- steering
- lap time
- normalized track position
- tyre state
- brake state
- fuel
- flags

If a simulator does not provide a field, leave it as `None`. Do not add simulator-specific branching to the web app for ordinary telemetry display.

## Simulator-Specific Data

V1 stores normalized telemetry first. Raw simulator packets may be added later behind an explicit debug/raw-capture setting. Raw capture should not be required for normal dashboards or MCP tools.
