use chrono::Utc;
use telemetry_core::{Sim, TelemetryFrame};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AccSharedMemoryError {
    #[error("ACC shared memory is only implemented on Windows")]
    UnsupportedPlatform,
    #[error("ACC shared memory block is unavailable: {0}")]
    Unavailable(&'static str),
    #[error("ACC packet changed during read")]
    PacketChanged,
}

#[derive(Debug, Clone)]
pub struct AccFrame {
    pub speed_kph: f32,
    pub rpm: u32,
    pub gear: i8,
    pub throttle: f32,
    pub brake: f32,
    pub lap_time_ms: u32,
}

impl AccFrame {
    pub fn into_telemetry_frame(self) -> TelemetryFrame {
        TelemetryFrame {
            sim: Sim::Acc,
            timestamp: Utc::now(),
            session: None,
            vehicle: None,
            speed_kph: Some(self.speed_kph),
            rpm: Some(self.rpm),
            gear: Some(self.gear),
            throttle: Some(self.throttle),
            brake: Some(self.brake),
            clutch: None,
            steering: None,
            lap_time_ms: Some(self.lap_time_ms),
            normalized_track_position: None,
            tyres: None,
            brakes: None,
            fuel: None,
            flags: Vec::new(),
        }
    }
}

pub struct AccSharedMemory;

impl AccSharedMemory {
    pub fn connect() -> Result<Self, AccSharedMemoryError> {
        platform::connect()
    }

    pub fn read_frame(&mut self) -> Result<Option<AccFrame>, AccSharedMemoryError> {
        platform::read_frame(self)
    }
}

#[cfg(windows)]
mod platform {
    use super::{AccFrame, AccSharedMemory, AccSharedMemoryError};

    pub fn connect() -> Result<AccSharedMemory, AccSharedMemoryError> {
        // TODO: map Local\\acpmf_physics, Local\\acpmf_graphics, and Local\\acpmf_static.
        Err(AccSharedMemoryError::Unavailable("not implemented yet"))
    }

    pub fn read_frame(
        _reader: &mut AccSharedMemory,
    ) -> Result<Option<AccFrame>, AccSharedMemoryError> {
        // TODO: implement packet_id double-read safety before returning clean typed frames.
        Err(AccSharedMemoryError::Unavailable("not implemented yet"))
    }
}

#[cfg(not(windows))]
mod platform {
    use super::{AccFrame, AccSharedMemory, AccSharedMemoryError};

    pub fn connect() -> Result<AccSharedMemory, AccSharedMemoryError> {
        Err(AccSharedMemoryError::UnsupportedPlatform)
    }

    pub fn read_frame(
        _reader: &mut AccSharedMemory,
    ) -> Result<Option<AccFrame>, AccSharedMemoryError> {
        Err(AccSharedMemoryError::UnsupportedPlatform)
    }
}
