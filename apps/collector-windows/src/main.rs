use anyhow::Result;
use clap::{Parser, ValueEnum};
use collector_core::{CollectorConfig, TelemetryUploader};
use uuid::Uuid;

#[derive(Debug, Parser)]
#[command(author, version, about = "Sim telemetry collector")]
struct Cli {
    #[arg(long)]
    server: String,
    #[arg(long, env = "SIM_TELEMETRY_TOKEN")]
    token: String,
    #[arg(long, value_enum, default_value_t = SimArg::Acc)]
    sim: SimArg,
    #[arg(long, default_value_t = Uuid::new_v4())]
    session_id: Uuid,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum SimArg {
    Acc,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let uploader = TelemetryUploader::new(CollectorConfig {
        server: cli.server,
        token: cli.token,
        session_id: cli.session_id,
    });

    match cli.sim {
        SimArg::Acc => run_acc_collector(uploader).await,
    }
}

async fn run_acc_collector(_uploader: TelemetryUploader) -> Result<()> {
    let _reader = acc_shared_memory::AccSharedMemory::connect()?;
    println!("ACC shared-memory reader connected. Upload loop is next.");
    Ok(())
}
