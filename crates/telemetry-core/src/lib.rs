use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Sim {
    Acc,
    Iracing,
    #[serde(rename = "assetto_corsa")]
    AssettoCorsa,
    Generic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TelemetryFrame {
    pub sim: Sim,
    pub timestamp: DateTime<Utc>,
    pub session: Option<SessionInfo>,
    pub vehicle: Option<VehicleInfo>,
    pub speed_kph: Option<f32>,
    pub rpm: Option<u32>,
    pub gear: Option<i8>,
    pub throttle: Option<f32>,
    pub brake: Option<f32>,
    pub clutch: Option<f32>,
    pub steering: Option<f32>,
    pub lap_time_ms: Option<u32>,
    pub normalized_track_position: Option<f32>,
    pub tyres: Option<TyreSet>,
    pub brakes: Option<BrakeSet>,
    pub fuel: Option<FuelInfo>,
    pub flags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionInfo {
    pub id: Option<Uuid>,
    pub track: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VehicleInfo {
    pub model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TyreSet {
    pub front_left: TyreState,
    pub front_right: TyreState,
    pub rear_left: TyreState,
    pub rear_right: TyreState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TyreState {
    pub pressure_psi: Option<f32>,
    pub temperature_c: Option<f32>,
    pub wear: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrakeSet {
    pub front_left_temperature_c: Option<f32>,
    pub front_right_temperature_c: Option<f32>,
    pub rear_left_temperature_c: Option<f32>,
    pub rear_right_temperature_c: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FuelInfo {
    pub liters: Option<f32>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TelemetryBatch {
    pub sim: Sim,
    pub session_id: Uuid,
    pub samples: Vec<TelemetryFrame>,
}
