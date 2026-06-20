use anyhow::Result;
use async_trait::async_trait;
use telemetry_core::{Sim, TelemetryBatch, TelemetryFrame};
use uuid::Uuid;

#[async_trait]
pub trait SimAdapter {
    async fn connect(&mut self) -> Result<()>;
    fn sim(&self) -> Sim;
    async fn next_frame(&mut self) -> Result<Option<TelemetryFrame>>;
}

#[derive(Debug, Clone)]
pub struct CollectorConfig {
    pub server: String,
    pub token: String,
    pub session_id: Uuid,
}

#[derive(Clone)]
pub struct TelemetryUploader {
    client: reqwest::Client,
    config: CollectorConfig,
}

impl TelemetryUploader {
    pub fn new(config: CollectorConfig) -> Self {
        Self {
            client: reqwest::Client::new(),
            config,
        }
    }

    pub async fn upload_batch(&self, samples: Vec<TelemetryFrame>) -> Result<()> {
        if samples.is_empty() {
            return Ok(());
        }

        let batch = TelemetryBatch {
            sim: samples[0].sim,
            session_id: self.config.session_id,
            samples,
        };

        self.client
            .post(format!(
                "{}/api/ingest/telemetry/batch",
                self.config.server.trim_end_matches('/')
            ))
            .bearer_auth(&self.config.token)
            .json(&batch)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}
