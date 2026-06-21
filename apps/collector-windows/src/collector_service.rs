use anyhow::Result;
use collector_core::{CollectorConfig, TelemetryUploader};
use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};
use telemetry_core::{Sim, TelemetryFrame};
use tokio::{select, sync::watch, time};
use uuid::Uuid;

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

#[derive(Debug, Clone)]
pub struct ServiceConfig {
    pub server: String,
    pub token: String,
    pub sim: Sim,
}

#[derive(Debug, Clone)]
pub enum CollectorEvent {
    Status(String),
    Running,
    Stopped,
}

enum LoopOutcome {
    StopRequested,
    Disconnected,
}

pub async fn run(
    config: ServiceConfig,
    event_tx: Sender<CollectorEvent>,
    shutdown_rx: watch::Receiver<bool>,
) -> Result<()> {
    emit(&event_tx, CollectorEvent::Running);

    let uploader = TelemetryUploader::new(CollectorConfig {
        server: config.server.clone(),
        token: config.token.clone(),
        session_id: Uuid::new_v4(),
    });
    let uploads_enabled = uploader.is_enabled();

    if uploads_enabled {
        emit(
            &event_tx,
            CollectorEvent::Status(format!("Uploading to {}.", config.server)),
        );
    } else {
        emit(
            &event_tx,
            CollectorEvent::Status(
                "Local mode: no server/token configured; telemetry will not be uploaded."
                    .to_string(),
            ),
        );
    }

    let result = match config.sim {
        Sim::Acc => {
            run_acc_service(
                uploader,
                config.sim,
                uploads_enabled,
                event_tx.clone(),
                shutdown_rx,
            )
            .await
        }
        other => {
            emit(
                &event_tx,
                CollectorEvent::Status(format!("Simulator {:?} is not implemented yet.", other)),
            );
            Ok(())
        }
    };

    if let Err(error) = &result {
        emit(
            &event_tx,
            CollectorEvent::Status(format!("Collector stopped: {error}")),
        );
    }

    emit(&event_tx, CollectorEvent::Stopped);
    result
}

async fn run_acc_service(
    uploader: TelemetryUploader,
    sim: Sim,
    uploads_enabled: bool,
    event_tx: Sender<CollectorEvent>,
    mut shutdown_rx: watch::Receiver<bool>,
) -> Result<()> {
    while !is_shutdown(&shutdown_rx) {
        emit(
            &event_tx,
            CollectorEvent::Status("Waiting for ACC shared memory...".to_string()),
        );

        let mut reader = match connect_acc(&event_tx, &mut shutdown_rx).await {
            Some(reader) => reader,
            None => break,
        };

        let (car_name, track_name) = {
            let frame = reader.read_frame().ok().flatten();
            let car = frame
                .as_ref()
                .map(|f| f.car_model.clone())
                .filter(|s| !s.is_empty());
            let track = frame
                .as_ref()
                .map(|f| f.track_name.clone())
                .filter(|s| !s.is_empty());
            (car, track)
        };

        let session_id = match uploader.start_session(sim, car_name, track_name).await {
            Ok(id) => {
                let status = if uploads_enabled {
                    format!("Session started: {id}")
                } else {
                    format!("Local session started: {id}")
                };
                emit(&event_tx, CollectorEvent::Status(status));
                id
            }
            Err(error) => {
                emit(
                    &event_tx,
                    CollectorEvent::Status(format!(
                        "Session start failed ({error}); collecting locally."
                    )),
                );
                Uuid::new_v4()
            }
        };

        let session_uploader = uploader.clone().with_session_id(session_id);
        let heartbeat_uploader = session_uploader.clone();
        let heartbeat_events = event_tx.clone();
        let mut heartbeat_shutdown = shutdown_rx.clone();

        let heartbeat = tokio::spawn(async move {
            let mut interval = time::interval(HEARTBEAT_INTERVAL);
            loop {
                select! {
                    changed = heartbeat_shutdown.changed() => {
                        if changed.is_err() || is_shutdown(&heartbeat_shutdown) {
                            break;
                        }
                    }
                    _ = interval.tick() => {
                        if let Err(error) = heartbeat_uploader.send_heartbeat(sim, "uploading", None).await {
                            emit(
                                &heartbeat_events,
                                CollectorEvent::Status(format!("Heartbeat failed: {error}")),
                            );
                        }
                    }
                }
            }
        });

        let outcome =
            collect_loop(&mut reader, &session_uploader, &event_tx, &mut shutdown_rx).await;

        heartbeat.abort();

        if let Err(error) = session_uploader.end_session(session_id).await {
            emit(
                &event_tx,
                CollectorEvent::Status(format!("Failed to end session: {error}")),
            );
        } else if uploads_enabled {
            emit(
                &event_tx,
                CollectorEvent::Status(format!("Session ended: {session_id}")),
            );
        } else {
            emit(
                &event_tx,
                CollectorEvent::Status(format!("Local session ended: {session_id}")),
            );
        }

        match outcome {
            LoopOutcome::StopRequested => break,
            LoopOutcome::Disconnected => {
                if sleep_or_shutdown(&mut shutdown_rx, RECONNECT_DELAY).await {
                    break;
                }
            }
        }
    }

    Ok(())
}

async fn connect_acc(
    event_tx: &Sender<CollectorEvent>,
    shutdown_rx: &mut watch::Receiver<bool>,
) -> Option<acc_shared_memory::AccSharedMemory> {
    loop {
        if is_shutdown(shutdown_rx) {
            return None;
        }

        match acc_shared_memory::AccSharedMemory::connect() {
            Ok(reader) => {
                emit(
                    event_tx,
                    CollectorEvent::Status("ACC shared memory connected.".to_string()),
                );
                return Some(reader);
            }
            Err(error) => {
                emit(
                    event_tx,
                    CollectorEvent::Status(format!(
                        "ACC not available ({error}); retrying in {}s.",
                        RECONNECT_DELAY.as_secs()
                    )),
                );
                if sleep_or_shutdown(shutdown_rx, RECONNECT_DELAY).await {
                    return None;
                }
            }
        }
    }
}

async fn collect_loop(
    reader: &mut acc_shared_memory::AccSharedMemory,
    uploader: &TelemetryUploader,
    event_tx: &Sender<CollectorEvent>,
    shutdown_rx: &mut watch::Receiver<bool>,
) -> LoopOutcome {
    let mut batch: Vec<TelemetryFrame> = Vec::with_capacity(BATCH_SIZE);
    let mut last_flush = Instant::now();
    let mut poll_interval = time::interval(POLL_INTERVAL);

    let outcome = loop {
        select! {
            changed = shutdown_rx.changed() => {
                if changed.is_err() || is_shutdown(shutdown_rx) {
                    emit(event_tx, CollectorEvent::Status("Stopping collector...".to_string()));
                    break LoopOutcome::StopRequested;
                }
            }
            _ = poll_interval.tick() => {
                match reader.read_frame() {
                    Ok(Some(acc_frame)) => {
                        batch.push(acc_frame.into_telemetry_frame());
                    }
                    Ok(None) => {
                        flush(&mut batch, uploader, event_tx).await;
                        emit(
                            event_tx,
                            CollectorEvent::Status("ACC is open but not in an active session.".to_string()),
                        );
                        if sleep_or_shutdown(shutdown_rx, RECONNECT_DELAY).await {
                            break LoopOutcome::StopRequested;
                        }
                        continue;
                    }
                    Err(error) => {
                        flush(&mut batch, uploader, event_tx).await;
                        emit(
                            event_tx,
                            CollectorEvent::Status(format!("ACC disconnected: {error}")),
                        );
                        break LoopOutcome::Disconnected;
                    }
                }

                if batch.len() >= BATCH_SIZE || last_flush.elapsed() >= FLUSH_INTERVAL {
                    flush(&mut batch, uploader, event_tx).await;
                    last_flush = Instant::now();
                }
            }
        }
    };

    flush(&mut batch, uploader, event_tx).await;
    outcome
}

async fn flush(
    batch: &mut Vec<TelemetryFrame>,
    uploader: &TelemetryUploader,
    event_tx: &Sender<CollectorEvent>,
) {
    if batch.is_empty() {
        return;
    }

    let frames = std::mem::take(batch);
    let count = frames.len();

    let uploads_enabled = uploader.is_enabled();

    if let Err(error) = uploader.upload_batch(frames).await {
        emit(
            event_tx,
            CollectorEvent::Status(format!("Upload failed for {count} frames: {error}")),
        );
    } else if uploads_enabled {
        emit(
            event_tx,
            CollectorEvent::Status(format!("Uploaded {count} frames.")),
        );
    } else {
        emit(
            event_tx,
            CollectorEvent::Status(format!("Collected {count} frames locally.")),
        );
    }
}

async fn sleep_or_shutdown(shutdown_rx: &mut watch::Receiver<bool>, duration: Duration) -> bool {
    if is_shutdown(shutdown_rx) {
        return true;
    }

    loop {
        select! {
            changed = shutdown_rx.changed() => {
                if changed.is_err() || is_shutdown(shutdown_rx) {
                    return true;
                }
            }
            _ = time::sleep(duration) => return false,
        }
    }
}

fn is_shutdown(shutdown_rx: &watch::Receiver<bool>) -> bool {
    *shutdown_rx.borrow()
}

fn emit(event_tx: &Sender<CollectorEvent>, event: CollectorEvent) {
    let _ = event_tx.send(event);
}
