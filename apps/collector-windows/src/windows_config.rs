use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};
use telemetry_core::Sim;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    pub server: String,
    pub token: String,
    pub sim: Sim,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: "http://localhost:3000".to_string(),
            token: String::new(),
            sim: Sim::Acc,
        }
    }
}

impl AppConfig {
    pub fn is_ready(&self) -> bool {
        !self.server.trim().is_empty()
    }

    pub fn load() -> Result<Self> {
        let path = config_path()?;
        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(&path)
            .with_context(|| format!("failed to read config from {}", path.display()))?;

        serde_json::from_str(&contents)
            .with_context(|| format!("failed to parse config from {}", path.display()))
    }

    pub fn save(&self) -> Result<()> {
        let path = config_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("failed to create config directory {}", parent.display())
            })?;
        }

        let contents = serde_json::to_string_pretty(self).context("failed to serialize config")?;
        fs::write(&path, contents)
            .with_context(|| format!("failed to write config to {}", path.display()))
    }
}

fn config_path() -> Result<PathBuf> {
    if let Ok(appdata) = env::var("APPDATA") {
        return Ok(PathBuf::from(appdata)
            .join("race-agent")
            .join("collector-config.json"));
    }

    let cwd = env::current_dir().context("failed to resolve current directory")?;
    Ok(cwd.join(".race-agent-collector.json"))
}
