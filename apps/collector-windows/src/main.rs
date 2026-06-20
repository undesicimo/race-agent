//! Windows ACC telemetry collector.
//!
//! Lifecycle
//! ─────────
//! 1. Wait for ACC shared-memory to become available (retry every second).
//! 2. POST /api/ingest/session/start  → receive server-assigned sessionId.
//! 3. Spawn a heartbeat task (every 5 s).
//! 4. Poll ACC at ~60 Hz, batch frames, and POST /api/ingest/telemetry/batch
//!    whenever the batch reaches BATCH_SIZE or the flush interval elapses.
//! 5. On Ctrl-C or ACC disconnect: flush remaining frames, POST session/end.

use anyhow::Result;
use clap::{Parser, ValueEnum};
use collector_core::{CollectorConfig, TelemetryUploader};
use std::time::{Duration, Instant};
use telemetry_core::{Sim, TelemetryFrame};
use tokio::{signal, time};
use uuid::Uuid;

// ── Configuration ─────────────────────────────────────────────────────────────

/// Maximum frames buffered before an upload is forced.
const BATCH_SIZE: usize = 50;
/// Maximum time between uploads (even if batch is not full).
const FLUSH_INTERVAL: Duration = Duration::from_millis(500);
/// How often to send a heartbeat to the server.
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How often to poll the shared-memory regions (~60 Hz).
const POLL_INTERVAL: Duration = Duration::from_millis(16);
/// How long to wait between retries while ACC is not running.
const RECONNECT_DELAY: Duration = Duration::from_secs(1);

// ── CLI ───────────────────────────────────────────────────────────────────────

#[derive(Debug, Parser)]
#[command(author, version, about = "ACC telemetry collector for Windows")]
struct Cli {
    /// Base URL of the sim-telemetry server (e.g. http://localhost:3000).
    #[arg(long)]
    server: String,

    /// Bearer token for ingest authentication.
    #[arg(long, env = "SIM_TELEMETRY_TOKEN")]
    token: String,

    /// Simulator to collect from.
    #[arg(long, value_enum, default_value_t = SimArg::Acc)]
    sim: SimArg,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum SimArg {
    Acc,
}

impl From<SimArg> for Sim {
    fn from(s: SimArg) -> Self {
        match s {
            SimArg::Acc => Sim::Acc,
        }
    }
}

// ── Entry point ───────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let sim: Sim = cli.sim.into();

    // Build uploader with a temporary session ID; it will be replaced after
    // start_session succeeds.
    let uploader = TelemetryUploader::new(CollectorConfig {
        server: cli.server.clone(),
        token: cli.token.clone(),
        session_id: Uuid::new_v4(),
    });

    match cli.sim {
        SimArg::Acc => run_acc(uploader, sim).await,
    }
}

// ── ACC collector loop ────────────────────────────────────────────────────────

async fn run_acc(uploader: TelemetryUploader, sim: Sim) -> Result<()> {
    eprintln!("[collector] Waiting for ACC shared-memory…");

    // ── Step 1: connect (retry until ACC is running) ──────────────────────────
    let mut reader = loop {
        match acc_shared_memory::AccSharedMemory::connect() {
            Ok(r) => {
                eprintln!("[collector] ACC shared-memory connected.");
                break r;
            }
            Err(e) => {
                eprintln!("[collector] {e} – retrying in {}s…", RECONNECT_DELAY.as_secs());
                time::sleep(RECONNECT_DELAY).await;
            }
        }
    };

    // Read one frame to pull initial car/track metadata for the session start.
    let (car_name, track_name) = {
        let frame = reader.read_frame().ok().flatten();
        let car   = frame.as_ref().map(|f| f.car_model.clone()).filter(|s| !s.is_empty());
        let track = frame.as_ref().map(|f| f.track_name.clone()).filter(|s| !s.is_empty());
        (car, track)
    };

    // ── Step 2: start session ─────────────────────────────────────────────────
    let session_id = match uploader.start_session(sim, car_name, track_name).await {
        Ok(id) => {
            eprintln!("[collector] Session started: {id}");
            id
        }
        Err(e) => {
            eprintln!("[collector] Failed to start session: {e}");
            // Continue with a local UUID; data will still be collected.
            Uuid::new_v4()
        }
    };

    let uploader = uploader.with_session_id(session_id);

    // ── Step 3: heartbeat task ────────────────────────────────────────────────
    let hb_uploader = uploader.clone();
    let hb_handle = tokio::spawn(async move {
        let mut interval = time::interval(HEARTBEAT_INTERVAL);
        loop {
            interval.tick().await;
            if let Err(e) = hb_uploader.send_heartbeat(sim, "uploading", None).await {
                eprintln!("[heartbeat] {e}");
            }
        }
    });

    // ── Step 4: collect & upload loop ─────────────────────────────────────────
    let result = collect_loop(&mut reader, &uploader, sim).await;

    hb_handle.abort();

    // ── Step 5: end session ───────────────────────────────────────────────────
    if let Err(e) = uploader.end_session(session_id).await {
        eprintln!("[collector] Failed to end session: {e}");
    } else {
        eprintln!("[collector] Session {session_id} ended.");
    }

    result
}

async fn collect_loop(
    reader: &mut acc_shared_memory::AccSharedMemory,
    uploader: &TelemetryUploader,
    sim: Sim,
) -> Result<()> {
    let mut batch: Vec<TelemetryFrame> = Vec::with_capacity(BATCH_SIZE);
    let mut last_flush = Instant::now();
    let mut poll_interval = time::interval(POLL_INTERVAL);

    let ctrl_c = signal::ctrl_c();
    tokio::pin!(ctrl_c);

    loop {
        tokio::select! {
            _ = poll_interval.tick() => {
                match reader.read_frame() {
                    Ok(Some(acc_frame)) => {
                        batch.push(acc_frame.into_telemetry_frame());
                    }
                    Ok(None) => {
                        // ACC is not in a live session; flush and wait.
                        flush(&mut batch, uploader).await;
                        eprintln!("[collector] ACC session ended or not started – waiting…");
                        time::sleep(RECONNECT_DELAY).await;
                        continue;
                    }
                    Err(e) => {
                        eprintln!("[collector] Read error: {e}");
                        break;
                    }
                }

                let should_flush =
                    batch.len() >= BATCH_SIZE || last_flush.elapsed() >= FLUSH_INTERVAL;

                if should_flush {
                    flush(&mut batch, uploader).await;
                    last_flush = Instant::now();
                }
            }
            _ = &mut ctrl_c => {
                eprintln!("\n[collector] Ctrl-C received – shutting down.");
                break;
            }
        }
    }

    // Flush any remaining frames before exiting.
    flush(&mut batch, uploader).await;
    Ok(())
}

async fn flush(batch: &mut Vec<TelemetryFrame>, uploader: &TelemetryUploader) {
    if batch.is_empty() {
        return;
    }
    let frames = std::mem::take(batch);
    let count = frames.len();
    if let Err(e) = uploader.upload_batch(frames).await {
        eprintln!("[upload] Failed to upload {count} frames: {e}");
    } else {
        eprintln!("[upload] Uploaded {count} frames.");
    }
}
