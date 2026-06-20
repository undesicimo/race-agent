use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
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

    pub fn is_enabled(&self) -> bool {
        !self.config.server.trim().is_empty() && !self.config.token.trim().is_empty()
    }

    pub async fn upload_batch(&self, samples: Vec<TelemetryFrame>) -> Result<()> {
        if samples.is_empty() {
            return Ok(());
        }

        if !self.is_enabled() {
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

    /// Register a new session with the server and return the server-assigned session UUID.
    pub async fn start_session(
        &self,
        sim: Sim,
        car_name: Option<String>,
        track_name: Option<String>,
    ) -> Result<Uuid> {
        if !self.is_enabled() {
            return Ok(self.config.session_id);
        }

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Req<'a> {
            sim: Sim,
            #[serde(skip_serializing_if = "Option::is_none")]
            car_name: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            track_name: Option<&'a str>,
        }

        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Resp {
            session_id: Uuid,
        }

        let body = Req {
            sim,
            car_name: car_name.as_deref(),
            track_name: track_name.as_deref(),
        };

        let resp: Resp = self
            .client
            .post(format!(
                "{}/api/ingest/session/start",
                self.config.server.trim_end_matches('/')
            ))
            .bearer_auth(&self.config.token)
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
            .context("failed to parse session/start response")?;

        Ok(resp.session_id)
    }

    /// Notify the server that the session has ended.
    pub async fn end_session(&self, session_id: Uuid) -> Result<()> {
        if !self.is_enabled() {
            return Ok(());
        }

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Req {
            session_id: Uuid,
        }

        self.client
            .post(format!(
                "{}/api/ingest/session/end",
                self.config.server.trim_end_matches('/')
            ))
            .bearer_auth(&self.config.token)
            .json(&Req { session_id })
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Send a heartbeat to the server.
    pub async fn send_heartbeat(
        &self,
        sim: Sim,
        status: &str,
        message: Option<&str>,
    ) -> Result<()> {
        if !self.is_enabled() {
            return Ok(());
        }

        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Req<'a> {
            sim: Sim,
            status: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            message: Option<&'a str>,
            timestamp: String,
        }

        let body = Req {
            sim,
            status,
            message,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        self.client
            .post(format!(
                "{}/api/ingest/heartbeat",
                self.config.server.trim_end_matches('/')
            ))
            .bearer_auth(&self.config.token)
            .json(&body)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    /// Return a clone of this uploader with a different session ID.
    pub fn with_session_id(mut self, session_id: Uuid) -> Self {
        self.config.session_id = session_id;
        self
    }
}
