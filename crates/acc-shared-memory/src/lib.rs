//! ACC shared-memory reader.
//!
//! Connects to the three shared-memory regions that Assetto Corsa Competizione
//! exposes on Windows (`acpmf_physics`, `acpmf_graphics`, `acpmf_static`) and
//! produces typed [`AccFrame`] values ready to be lifted into [`TelemetryFrame`].
//!
//! All C structs use `#[repr(C, packed(4))]` to match the `#pragma pack(4)` that
//! ACC's own Win32 SDK headers declare.

use chrono::Utc;
use telemetry_core::{BrakeSet, FuelInfo, SessionInfo, Sim, TelemetryFrame, TyreSet, TyreState, VehicleInfo};
use thiserror::Error;

// ── Error type ────────────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum AccSharedMemoryError {
    #[error("ACC shared memory is only implemented on Windows")]
    UnsupportedPlatform,
    #[error("ACC shared memory block is unavailable: {0}")]
    Unavailable(&'static str),
}

// ── ACC page structs (binary layout must match the game) ──────────────────────

/// Updated at every physics step.
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct PageFilePhysics {
    pub packet_id: i32,
    pub gas: f32,
    pub brake: f32,
    /// Fuel remaining in kg.
    pub fuel: f32,
    /// 0 = reverse, 1 = neutral, 2 = 1st gear, …
    pub gear: i32,
    pub rpm: i32,
    pub steer_angle: f32,
    pub speed_kmh: f32,
    pub velocity: [f32; 3],
    pub acc_g: [f32; 3],
    pub wheel_slip: [f32; 4],
    pub wheel_load: [f32; 4],
    pub wheels_pressure: [f32; 4],
    pub wheel_angular_speed: [f32; 4],
    pub tyre_wear: [f32; 4],
    pub tyre_dirty_level: [f32; 4],
    pub tyre_core_temperature: [f32; 4],
    pub camber_rad: [f32; 4],
    pub suspension_travel: [f32; 4],
    pub drs: f32,
    pub tc: f32,
    pub heading: f32,
    pub pitch: f32,
    pub roll: f32,
    pub cg_height: f32,
    pub car_damage: [f32; 5],
    pub number_of_tyres_out: i32,
    pub pit_limiter_on: i32,
    pub abs: f32,
    pub kers_charge: f32,
    pub kers_input: f32,
    pub auto_shifter_on: i32,
    pub ride_height: [f32; 2],
    pub turbo_boost: f32,
    pub ballast: f32,
    pub air_density: f32,
    pub air_temp: f32,
    pub road_temp: f32,
    pub local_angular_vel: [f32; 3],
    pub final_ff: f32,
    pub performance_meter: f32,
    pub engine_brake: i32,
    pub ers_recovery_level: i32,
    pub ers_power_level: i32,
    pub ers_heat_charging: i32,
    pub ers_is_charging: i32,
    pub kers_current_kj: f32,
    pub drs_available: i32,
    pub drs_enabled: i32,
    pub brake_temp: [f32; 4],
    pub clutch: f32,
    pub tyre_temp_i: [f32; 4],
    pub tyre_temp_m: [f32; 4],
    pub tyre_temp_o: [f32; 4],
    pub is_ai_controlled: i32,
    pub tyre_contact_point: [[f32; 3]; 4],
    pub tyre_contact_normal: [[f32; 3]; 4],
    pub tyre_contact_heading: [[f32; 3]; 4],
    pub brake_bias: f32,
    pub local_velocity: [f32; 3],
    pub p2p_activations: i32,
    pub p2p_status: i32,
    pub current_max_rpm: i32,
    pub mz: [f32; 4],
    pub fx: [f32; 4],
    pub fy: [f32; 4],
    pub slip_ratio: [f32; 4],
    pub slip_angle: [f32; 4],
    pub tc_in_action: i32,
    pub abs_in_action: i32,
    pub suspension_damage: [f32; 4],
    pub tyre_temp: [f32; 4],
    pub water_temp: f32,
    pub brake_pressure: [f32; 4],
    pub front_brake_compound: i32,
    pub rear_brake_compound: i32,
    pub pad_life: [f32; 4],
    pub disc_life: [f32; 4],
    pub ignition_on: i32,
    pub starter_engine_on: i32,
    pub is_engine_running: i32,
    pub kerb_vibration: f32,
    pub slip_vibrations: f32,
    pub g_vibrations: f32,
    pub abs_vibrations: f32,
}

/// Updated at every graphical step (race position, flags, session state).
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct PageFileGraphics {
    pub packet_id: i32,
    /// 0 = Off, 1 = Replay, 2 = Live, 3 = Pause.
    pub status: i32,
    pub session: i32,
    pub current_time: [u16; 15],
    pub last_time: [u16; 15],
    pub best_time: [u16; 15],
    pub split: [u16; 15],
    pub completed_laps: i32,
    pub position: i32,
    pub i_current_time: i32,
    pub i_last_time: i32,
    pub i_best_time: i32,
    pub session_time_left: f32,
    pub distance_traveled: f32,
    pub is_in_pit: i32,
    pub current_sector_index: i32,
    pub last_sector_time: i32,
    pub number_of_laps: i32,
    pub tyre_compound: [u16; 33],
    pub replay_time_multiplier: f32,
    pub normalized_car_position: f32,
    pub active_cars: i32,
    pub car_coordinates: [[f32; 3]; 60],
    pub car_id: [i32; 60],
    pub player_car_id: i32,
    pub penalty_time: f32,
    /// 0=None 1=Blue 2=Yellow 3=Black 4=White 5=Checkered 6=Penalty 7=Green 8=Orange.
    pub flag: i32,
    pub penalty: i32,
    pub ideal_line_on: i32,
    pub is_in_pit_lane: i32,
    pub surface_grip: f32,
    pub mandatory_pit_done: i32,
    pub wind_speed: f32,
    pub wind_direction: f32,
    pub is_setup_menu_visible: i32,
    pub main_display_index: i32,
    pub secondary_display_index: i32,
    pub tc: i32,
    pub tc_cut: i32,
    pub engine_map: i32,
    pub abs: i32,
    pub fuel_used_per_lap: f32,
    pub rain_lights: i32,
    pub flashing_lights: i32,
    pub lights_stage: i32,
    pub exhaust_temperature: f32,
    pub wiper_lv: i32,
    pub driver_stint_total_time_left: i32,
    pub driver_stint_time_left: i32,
    pub rain_tyres: i32,
    pub session_index: i32,
    pub used_fuel: f32,
    pub delta_lap_time: [u16; 15],
    pub i_delta_lap_time: i32,
    pub estimated_lap_time: [u16; 15],
    pub i_estimated_lap_time: i32,
    pub is_delta_positive: i32,
    pub i_split: i32,
    pub is_valid_lap: i32,
    pub fuel_estimated_laps: f32,
    pub track_status: [u16; 33],
    pub missing_mandatory_pits: i32,
    pub clock: f32,
    pub direction_lights_left: i32,
    pub direction_lights_right: i32,
    pub global_yellow: i32,
    pub global_yellow1: i32,
    pub global_yellow2: i32,
    pub global_yellow3: i32,
    pub global_white: i32,
    pub global_green: i32,
    pub global_chequered: i32,
    pub global_red: i32,
    pub mfd_tyre_set: i32,
    pub mfd_fuel_to_add: f32,
    pub mfd_tyre_pressure_lf: f32,
    pub mfd_tyre_pressure_rf: f32,
    pub mfd_tyre_pressure_lr: f32,
    pub mfd_tyre_pressure_rr: f32,
    pub track_grip_status: i32,
    pub rain_intensity: i32,
    pub rain_intensity_in_10m: i32,
    pub rain_intensity_in_30m: i32,
    pub current_tyre_set: i32,
    pub strategy_tyre_set: i32,
    pub gap_ahead: i32,
    pub gap_behind: i32,
}

/// Static data that does not change during a session.
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct PageFileStatic {
    pub sm_version: [u16; 15],
    pub ac_version: [u16; 15],
    pub number_of_sessions: i32,
    pub num_cars: i32,
    pub car_model: [u16; 33],
    pub track: [u16; 33],
    pub player_name: [u16; 33],
    pub player_surname: [u16; 33],
    pub player_nick: [u16; 33],
    pub sector_count: i32,
    pub max_torque: f32,
    pub max_power: f32,
    pub max_rpm: i32,
    pub max_fuel: f32,
    pub suspension_max_travel: [f32; 4],
    pub tyre_radius: [f32; 4],
    pub max_turbo_boost: f32,
    pub deprecated_1: f32,
    pub deprecated_2: f32,
    pub penalties_enabled: i32,
    pub aid_fuel_rate: f32,
    pub aid_tyre_rate: f32,
    pub aid_mechanical_damage: f32,
    pub aid_allow_tyre_blankets: f32,
    pub aid_stability: f32,
    pub aid_auto_clutch: i32,
    pub aid_auto_blip: i32,
    pub has_drs: i32,
    pub has_ers: i32,
    pub has_kers: i32,
    pub kers_max_j: f32,
    pub engine_brake_settings_count: i32,
    pub ers_power_controller_count: i32,
    pub track_spline_length: f32,
    pub track_configuration: [u16; 33],
    pub ers_max_j: f32,
    pub is_timed_race: i32,
    pub has_extra_lap: i32,
    pub car_skin: [u16; 33],
    pub reversed_grid_positions: i32,
    pub pit_window_start: i32,
    pub pit_window_end: i32,
    pub is_online: i32,
    pub dry_tyres_name: [u16; 33],
    pub wet_tyres_name: [u16; 33],
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Decode a null-terminated UTF-16 LE string from an ACC page field.
fn decode_utf16(data: &[u16]) -> String {
    let end = data.iter().position(|&c| c == 0).unwrap_or(data.len());
    String::from_utf16_lossy(&data[..end]).into_owned()
}

// ── Decoded frame ─────────────────────────────────────────────────────────────

/// A fully-decoded ACC telemetry snapshot.
#[derive(Debug, Clone)]
pub struct AccFrame {
    pub speed_kph: f32,
    pub rpm: u32,
    /// Standard convention: -1 = reverse, 0 = neutral, 1 = 1st gear, …
    pub gear: i8,
    pub throttle: f32,
    pub brake: f32,
    pub clutch: f32,
    pub steering: f32,
    /// Current lap time in milliseconds.
    pub lap_time_ms: u32,
    /// 0.0 = start/finish line, 1.0 = finish.
    pub normalized_track_position: f32,
    /// Fuel remaining in kilograms (ACC native).
    pub fuel_kg: f32,
    /// Tyre pressures in PSI [FL, FR, RL, RR].
    pub tyre_pressure: [f32; 4],
    /// Tyre core temperatures in °C [FL, FR, RL, RR].
    pub tyre_temp_c: [f32; 4],
    /// Brake disc temperatures in °C [FL, FR, RL, RR].
    pub brake_temp_c: [f32; 4],
    pub flag: i32,
    pub car_model: String,
    pub track_name: String,
    pub position: i32,
    pub completed_laps: i32,
}

impl AccFrame {
    pub fn into_telemetry_frame(self) -> TelemetryFrame {
        let tyre = |i: usize| TyreState {
            pressure_psi: Some(self.tyre_pressure[i]),
            temperature_c: Some(self.tyre_temp_c[i]),
            wear: None,
        };

        let flags: Vec<String> = match self.flag {
            1 => vec!["blue".into()],
            2 => vec!["yellow".into()],
            3 => vec!["black".into()],
            4 => vec!["white".into()],
            5 => vec!["checkered".into()],
            6 => vec!["penalty".into()],
            7 => vec!["green".into()],
            8 => vec!["orange".into()],
            _ => vec![],
        };

        // ACC stores fuel in kg; convert to litres (petrol ~0.725 kg/L).
        let fuel_liters = self.fuel_kg / 0.725;

        TelemetryFrame {
            sim: Sim::Acc,
            timestamp: Utc::now(),
            session: Some(SessionInfo {
                id: None,
                track: if self.track_name.is_empty() { None } else { Some(self.track_name) },
            }),
            vehicle: Some(VehicleInfo {
                model: if self.car_model.is_empty() { None } else { Some(self.car_model) },
            }),
            speed_kph: Some(self.speed_kph),
            rpm: Some(self.rpm),
            gear: Some(self.gear),
            throttle: Some(self.throttle),
            brake: Some(self.brake),
            clutch: Some(self.clutch),
            steering: Some(self.steering),
            lap_time_ms: Some(self.lap_time_ms),
            normalized_track_position: Some(self.normalized_track_position),
            tyres: Some(TyreSet {
                front_left: tyre(0),
                front_right: tyre(1),
                rear_left: tyre(2),
                rear_right: tyre(3),
            }),
            brakes: Some(BrakeSet {
                front_left_temperature_c: Some(self.brake_temp_c[0]),
                front_right_temperature_c: Some(self.brake_temp_c[1]),
                rear_left_temperature_c: Some(self.brake_temp_c[2]),
                rear_right_temperature_c: Some(self.brake_temp_c[3]),
            }),
            fuel: Some(FuelInfo { liters: Some(fuel_liters) }),
            flags,
        }
    }
}

// ── Public reader ─────────────────────────────────────────────────────────────

pub struct AccSharedMemory(platform::AccInner);

impl AccSharedMemory {
    /// Open all three ACC shared-memory regions.
    /// Fails with [`AccSharedMemoryError::UnsupportedPlatform`] on non-Windows.
    /// Fails with [`AccSharedMemoryError::Unavailable`] if ACC is not running.
    pub fn connect() -> Result<Self, AccSharedMemoryError> {
        platform::connect()
    }

    /// Read the current telemetry snapshot.
    ///
    /// Returns `Ok(None)` when `status == Off` (ACC is not in a live session).
    /// Applies packet-ID double-read on the physics page to prevent torn reads.
    pub fn read_frame(&mut self) -> Result<Option<AccFrame>, AccSharedMemoryError> {
        platform::read_frame(&mut self.0)
    }
}

// ── Platform: Windows ─────────────────────────────────────────────────────────

#[cfg(windows)]
mod platform {
    use super::{AccFrame, AccSharedMemoryError, PageFileGraphics, PageFilePhysics, PageFileStatic, decode_utf16};
    use std::ffi::c_void;
    use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
    use windows_sys::Win32::System::Memory::{FILE_MAP_READ, MapViewOfFile, OpenFileMappingA, UnmapViewOfFile};

    struct SafeHandle(HANDLE);
    impl Drop for SafeHandle {
        fn drop(&mut self) { unsafe { CloseHandle(self.0); } }
    }

    struct SafeView(*const c_void);
    unsafe impl Send for SafeView {}
    unsafe impl Sync for SafeView {}
    impl Drop for SafeView {
        fn drop(&mut self) { unsafe { UnmapViewOfFile(self.0); } }
    }

    struct Mapping {
        _handle: SafeHandle,
        view: SafeView,
    }

    impl Mapping {
        fn open(name: &[u8]) -> Result<Self, AccSharedMemoryError> {
            let handle: HANDLE = unsafe { OpenFileMappingA(FILE_MAP_READ, 0, name.as_ptr()) };
            if handle == 0 || handle == INVALID_HANDLE_VALUE {
                return Err(AccSharedMemoryError::Unavailable("OpenFileMappingA failed – is ACC running?"));
            }
            let view = unsafe { MapViewOfFile(handle, FILE_MAP_READ, 0, 0, 0) };
            if view.is_null() {
                unsafe { CloseHandle(handle); }
                return Err(AccSharedMemoryError::Unavailable("MapViewOfFile failed"));
            }
            Ok(Self { _handle: SafeHandle(handle), view: SafeView(view) })
        }

        /// Copy the mapped region into `T` using an unaligned read.
        #[inline]
        unsafe fn read_copy<T: Copy>(&self) -> T {
            std::ptr::read_unaligned(self.view.0 as *const T)
        }
    }

    pub struct AccInner {
        physics: Mapping,
        graphics: Mapping,
        static_data: Mapping,
        car_model: String,
        track_name: String,
    }

    pub fn connect() -> Result<super::AccSharedMemory, AccSharedMemoryError> {
        let physics     = Mapping::open(b"Local\\acpmf_physics\0")?;
        let graphics    = Mapping::open(b"Local\\acpmf_graphics\0")?;
        let static_data = Mapping::open(b"Local\\acpmf_static\0")?;

        let page_static: PageFileStatic = unsafe { static_data.read_copy() };
        let car_model  = decode_utf16(&page_static.car_model);
        let track_name = decode_utf16(&page_static.track);

        Ok(super::AccSharedMemory(AccInner { physics, graphics, static_data, car_model, track_name }))
    }

    pub fn read_frame(inner: &mut AccInner) -> Result<Option<AccFrame>, AccSharedMemoryError> {
        // Status check – bail early when not in a live session.
        let gfx: PageFileGraphics = unsafe { inner.graphics.read_copy() };
        if gfx.status == 0 {
            return Ok(None);
        }

        // Physics double-read: spin until packet_id is stable across the memcpy.
        let phys: PageFilePhysics = {
            loop {
                let id_before: i32 = unsafe { std::ptr::read_unaligned(inner.physics.view.0 as *const i32) };
                let data: PageFilePhysics = unsafe { inner.physics.read_copy() };
                let id_after: i32 = unsafe { std::ptr::read_unaligned(inner.physics.view.0 as *const i32) };
                if id_before == id_after { break data; }
                std::hint::spin_loop();
            }
        };

        // Refresh static metadata (car/track can change between sessions).
        let page_static: PageFileStatic = unsafe { inner.static_data.read_copy() };
        inner.car_model  = decode_utf16(&page_static.car_model);
        inner.track_name = decode_utf16(&page_static.track);

        // ACC gear encoding: 0=reverse, 1=neutral, 2=1st … → -1/0/1 …
        let gear: i8 = (phys.gear - 1) as i8;

        Ok(Some(AccFrame {
            speed_kph:                  phys.speed_kmh,
            rpm:                        phys.rpm as u32,
            gear,
            throttle:                   phys.gas,
            brake:                      phys.brake,
            clutch:                     phys.clutch,
            steering:                   phys.steer_angle,
            lap_time_ms:                gfx.i_current_time.max(0) as u32,
            normalized_track_position:  gfx.normalized_car_position.clamp(0.0, 1.0),
            fuel_kg:                    phys.fuel,
            tyre_pressure:              phys.wheels_pressure,
            tyre_temp_c:                phys.tyre_core_temperature,
            brake_temp_c:               phys.brake_temp,
            flag:                       gfx.flag,
            car_model:                  inner.car_model.clone(),
            track_name:                 inner.track_name.clone(),
            position:                   gfx.position,
            completed_laps:             gfx.completed_laps,
        }))
    }
}

// ── Platform: non-Windows ─────────────────────────────────────────────────────

#[cfg(not(windows))]
mod platform {
    use super::{AccFrame, AccSharedMemoryError};

    pub struct AccInner;

    pub fn connect() -> Result<super::AccSharedMemory, AccSharedMemoryError> {
        Err(AccSharedMemoryError::UnsupportedPlatform)
    }

    pub fn read_frame(_: &mut AccInner) -> Result<Option<AccFrame>, AccSharedMemoryError> {
        Err(AccSharedMemoryError::UnsupportedPlatform)
    }
}
