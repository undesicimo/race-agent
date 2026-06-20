#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod collector_service;

#[cfg(target_os = "windows")]
mod windows_app;
#[cfg(target_os = "windows")]
mod windows_config;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use telemetry_core::Sim;

#[cfg(not(target_os = "windows"))]
use std::sync::mpsc;
#[cfg(not(target_os = "windows"))]
use tokio::{signal, sync::watch};

#[derive(Debug, Clone, Copy, ValueEnum)]
enum SimArg {
    Acc,
}

impl From<SimArg> for Sim {
    fn from(value: SimArg) -> Self {
        match value {
            SimArg::Acc => Sim::Acc,
        }
    }
}

#[cfg(target_os = "windows")]
#[derive(Debug, Parser)]
#[command(author, version, about = "Windows tray collector for sim telemetry")]
struct Cli {
    /// Override the server URL stored in the local collector config.
    #[arg(long)]
    server: Option<String>,

    /// Override the token stored in the local collector config.
    #[arg(long)]
    token: Option<String>,

    /// Show the settings window on launch.
    #[arg(long, default_value_t = false)]
    show: bool,
}

#[cfg(not(target_os = "windows"))]
#[derive(Debug, Parser)]
#[command(author, version, about = "ACC telemetry collector")]
struct Cli {
    /// Base URL of the sim-telemetry server (e.g. http://localhost:3000).
    #[arg(long)]
    server: String,

    /// Bearer token for ingest authentication.
    #[arg(long)]
    token: String,

    /// Simulator to collect from.
    #[arg(long, value_enum, default_value_t = SimArg::Acc)]
    sim: SimArg,
}

#[cfg(target_os = "windows")]
fn main() -> Result<()> {
    let cli = Cli::parse();

    windows_app::run(windows_app::LaunchOverrides {
        server: cli.server,
        token: cli.token,
        show_window: cli.show,
    })
}

#[cfg(not(target_os = "windows"))]
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let (event_tx, event_rx) = mpsc::channel();
    let (_stop_tx, stop_rx) = watch::channel(false);

    let event_thread = std::thread::spawn(move || {
        while let Ok(event) = event_rx.recv() {
            match event {
                collector_service::CollectorEvent::Status(message) => {
                    eprintln!("[collector] {message}")
                }
                collector_service::CollectorEvent::Running => eprintln!("[collector] Running."),
                collector_service::CollectorEvent::Stopped => break,
            }
        }
    });

    let service = collector_service::run(
        collector_service::ServiceConfig {
            server: cli.server,
            token: cli.token,
            sim: cli.sim.into(),
        },
        event_tx,
        stop_rx,
    );

    tokio::select! {
        result = service => {
            result?;
        }
        result = signal::ctrl_c() => {
            result?;
        }
    }

    let _ = event_thread.join();
    Ok(())
}
