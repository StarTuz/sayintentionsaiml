//! Telemetry - File-based communication with X-Plane
//!
//! Watches `~/.local/share/StratusATC/stratus_telemetry.json` for telemetry updates
//! and writes commands to `stratus_commands.jsonl`.

use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::mpsc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TelemetryError {
    #[error("Failed to read telemetry file: {0}")]
    ReadError(#[from] std::io::Error),
    #[error("Failed to parse telemetry JSON: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("File watcher error: {0}")]
    WatchError(#[from] notify::Error),
}

/// Aircraft telemetry from X-Plane
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Telemetry {
    pub timestamp: i64,
    pub simulator: String,
    pub aircraft: String,
    pub position: Position,
    pub orientation: Orientation,
    pub speed: Speed,
    pub radios: Radios,
    pub transponder: Transponder,
    pub state: FlightState,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Position {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude_msl_m: f64,
    pub altitude_agl_m: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Orientation {
    pub heading_mag: f32,
    pub heading_true: f32,
    pub pitch: f32,
    pub roll: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Speed {
    pub ground_speed_mps: f32,
    pub ias_kts: f32,
    pub tas_mps: f32,
    pub vertical_speed_fpm: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Radios {
    pub com1_hz: i32,
    pub com1_standby_hz: i32,
    pub com2_hz: i32,
    pub com2_standby_hz: i32,
    pub nav1_hz: i32,
    pub nav2_hz: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Transponder {
    pub code: i32,
    pub mode: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FlightState {
    pub on_ground: bool,
    pub paused: bool,
}

/// Watches the Telemetry input file for changes
pub struct TelemetryWatcher {
    data_dir: PathBuf,
    _watcher: RecommendedWatcher,
    receiver: mpsc::Receiver<Result<Event, notify::Error>>,
}

impl TelemetryWatcher {
    /// Create a new watcher for the Telemetry directory
    pub fn new() -> Result<Self, TelemetryError> {
        let data_dir = Self::get_data_dir();

        // Create directory if it doesn't exist
        std::fs::create_dir_all(&data_dir)?;

        let (tx, rx) = mpsc::channel();

        let mut watcher = RecommendedWatcher::new(
            move |res| {
                let _ = tx.send(res);
            },
            notify::Config::default(),
        )?;

        watcher.watch(&data_dir, RecursiveMode::NonRecursive)?;

        Ok(Self {
            data_dir,
            _watcher: watcher,
            receiver: rx,
        })
    }

    /// Get the platform-specific data directory
    fn get_data_dir() -> PathBuf {
        #[cfg(target_os = "linux")]
        {
            dirs::data_local_dir()
                .unwrap_or_else(|| PathBuf::from("/tmp"))
                .join("StratusATC")
        }
        #[cfg(target_os = "macos")]
        {
            dirs::data_dir()
                .unwrap_or_else(|| PathBuf::from("/tmp"))
                .join("StratusATC")
        }
        #[cfg(target_os = "windows")]
        {
            dirs::data_local_dir()
                .unwrap_or_else(|| PathBuf::from("C:\\StratusATC"))
                .join("StratusATC")
        }
    }

    /// Read the current telemetry from the input file
    pub fn read_telemetry(&self) -> Result<Telemetry, TelemetryError> {
        let path = self.data_dir.join("stratus_telemetry.json");
        let content = std::fs::read_to_string(&path)?;
        let telemetry: Telemetry = serde_json::from_str(&content)?;
        Ok(telemetry)
    }

    /// Check for file changes (non-blocking)
    pub fn poll(&self) -> Option<Result<Telemetry, TelemetryError>> {
        match self.receiver.try_recv() {
            Ok(Ok(_event)) => Some(self.read_telemetry()),
            Ok(Err(e)) => Some(Err(TelemetryError::WatchError(e))),
            Err(_) => None,
        }
    }
}

impl Default for TelemetryWatcher {
    fn default() -> Self {
        Self::new().expect("Failed to create Telemetry watcher")
    }
}
