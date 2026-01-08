//! Stratus Application - Main Iced App
//!
//! Elm-style architecture with Message passing.

use iced::widget::{
    column, container, horizontal_space, row, scrollable, text, text_input, Column,
};
use iced::{time, Element, Length, Subscription, Task, Theme};
use std::path::PathBuf;
use std::time::Duration;
use stratus_core::Telemetry;

/// Main application state
pub struct StratusApp {
    // UI State
    input_text: String,
    comm_log: Vec<CommEntry>,

    // Core state
    telemetry: Telemetry,
    connected: bool,
    last_telemetry_time: std::time::Instant,
    ollama_status: OllamaStatus,

    // Paths
    data_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub struct CommEntry {
    pub speaker: String,
    pub message: String,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum OllamaStatus {
    #[default]
    Unknown,
    Connected,
    Disconnected,
}

/// Messages that can be sent to the application
#[derive(Debug, Clone)]
pub enum Message {
    // UI Events
    InputChanged(String),
    SendMessage,

    // Background events
    TelemetryUpdated(Result<Telemetry, String>),
    OllamaStatusChanged(bool),

    // System
    Tick,
    CheckOllama,
}

impl StratusApp {
    /// Create new application with initial state
    pub fn new() -> (Self, Task<Message>) {
        let data_dir = Self::get_data_dir();

        let app = Self {
            input_text: String::new(),
            comm_log: vec![CommEntry {
                speaker: "SYSTEM".into(),
                message: "Stratus ATC initialized. Waiting for X-Plane connection...".into(),
            }],
            telemetry: Telemetry::default(),
            connected: false,
            last_telemetry_time: std::time::Instant::now(),
            ollama_status: OllamaStatus::Unknown,
            data_dir,
        };

        // Initial tasks
        let check_ollama = Task::perform(check_ollama_available(), Message::OllamaStatusChanged);

        (app, check_ollama)
    }

    fn get_data_dir() -> PathBuf {
        dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("StratusATC")
    }

    /// Handle messages (Elm-style update)
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::InputChanged(text) => {
                self.input_text = text;
                Task::none()
            }
            Message::SendMessage => {
                if !self.input_text.trim().is_empty() {
                    let pilot_msg = self.input_text.clone();
                    self.comm_log.push(CommEntry {
                        speaker: "PILOT".into(),
                        message: pilot_msg.clone(),
                    });
                    self.input_text.clear();

                    // TODO: Send to ATC engine (Phase 3)
                    self.comm_log.push(CommEntry {
                        speaker: "ATC".into(),
                        message: format!("Roger, {}", pilot_msg),
                    });
                }
                Task::none()
            }
            Message::TelemetryUpdated(result) => {
                match result {
                    Ok(telemetry) => {
                        self.telemetry = telemetry;
                        self.connected = true;
                        self.last_telemetry_time = std::time::Instant::now();
                    }
                    Err(_) => {
                        // Check if connection is stale (no update in 5 seconds)
                        if self.last_telemetry_time.elapsed() > Duration::from_secs(5) {
                            self.connected = false;
                        }
                    }
                }
                Task::none()
            }
            Message::OllamaStatusChanged(available) => {
                self.ollama_status = if available {
                    OllamaStatus::Connected
                } else {
                    OllamaStatus::Disconnected
                };
                Task::none()
            }
            Message::Tick => {
                // Read telemetry file
                let path = self.data_dir.join("stratus_telemetry.json");
                Task::perform(read_telemetry_file(path), Message::TelemetryUpdated)
            }
            Message::CheckOllama => {
                Task::perform(check_ollama_available(), Message::OllamaStatusChanged)
            }
        }
    }

    /// Render the view
    pub fn view(&self) -> Element<'_, Message> {
        let header = self.view_header();
        let main_content = self.view_main();
        let footer = self.view_footer();

        let content = column![header, main_content, footer]
            .spacing(10)
            .padding(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn view_header(&self) -> Element<'_, Message> {
        let status_text = if self.connected {
            text("● Connected to X-Plane").color([0.3, 0.9, 0.3])
        } else {
            text("○ Waiting for X-Plane...").color([0.6, 0.6, 0.6])
        };

        let ollama_text = match self.ollama_status {
            OllamaStatus::Connected => text("🧠 Ollama Ready").color([0.3, 0.9, 0.3]),
            OllamaStatus::Disconnected => text("⚠ Ollama Offline").color([0.9, 0.6, 0.3]),
            OllamaStatus::Unknown => text("? Checking Ollama...").color([0.6, 0.6, 0.6]),
        };

        row![
            text("STRATUS ATC").size(24),
            horizontal_space(),
            status_text,
            text(" | "),
            ollama_text,
        ]
        .spacing(10)
        .into()
    }

    fn view_main(&self) -> Element<'_, Message> {
        let telemetry_panel = self.view_telemetry();
        let comm_panel = self.view_comm_log();

        row![telemetry_panel, comm_panel]
            .spacing(20)
            .height(Length::Fill)
            .into()
    }

    fn view_telemetry(&self) -> Element<'_, Message> {
        let alt_ft = (self.telemetry.position.altitude_msl_m * 3.28084) as i32;
        let hdg = self.telemetry.orientation.heading_mag as i32;
        let spd = (self.telemetry.speed.ground_speed_mps * 1.94384) as i32;
        let ias = self.telemetry.speed.ias_kts as i32;

        let content = column![
            text("TELEMETRY").size(16),
            text(format!("ALT: {} ft", alt_ft)),
            text(format!("HDG: {}°", hdg)),
            text(format!("GS: {} kts", spd)),
            text(format!("IAS: {} kts", ias)),
            text(format!("XPDR: {:04}", self.telemetry.transponder.code)),
            text(""),
            text(format!(
                "COM1: {:.3}",
                self.telemetry.radios.com1_hz as f64 / 1_000_000.0
            )),
        ]
        .spacing(8)
        .padding(10);

        container(content).width(200).height(Length::Fill).into()
    }

    fn view_comm_log(&self) -> Element<'_, Message> {
        let entries: Vec<Element<'_, Message>> = self
            .comm_log
            .iter()
            .map(|entry| {
                let color = match entry.speaker.as_str() {
                    "PILOT" => [0.5, 0.8, 1.0],
                    "ATC" => [0.3, 1.0, 0.5],
                    "SYSTEM" => [0.7, 0.7, 0.7],
                    _ => [1.0, 1.0, 1.0],
                };
                text(format!("{}: {}", entry.speaker, entry.message))
                    .color(color)
                    .into()
            })
            .collect();

        let log = Column::with_children(entries).spacing(4);

        let scroll = scrollable(log).height(Length::Fill);

        let input = text_input("Type message...", &self.input_text)
            .on_input(Message::InputChanged)
            .on_submit(Message::SendMessage)
            .padding(10);

        column![text("COMMUNICATIONS").size(16), scroll, input,]
            .spacing(10)
            .width(Length::Fill)
            .into()
    }

    fn view_footer(&self) -> Element<'_, Message> {
        let lat = self.telemetry.position.latitude;
        let lon = self.telemetry.position.longitude;

        row![
            text(format!("Position: {:.4}°, {:.4}°", lat, lon)).size(12),
            horizontal_space(),
            text("Stratus ATC v0.1.0").size(12),
        ]
        .into()
    }

    /// Get the theme
    pub fn theme(&self) -> Theme {
        Theme::Dark
    }

    /// Subscriptions for background tasks
    pub fn subscription(&self) -> Subscription<Message> {
        // Poll telemetry every 500ms
        let telemetry_tick = time::every(Duration::from_millis(500)).map(|_| Message::Tick);

        // Check Ollama every 10 seconds
        let ollama_check = time::every(Duration::from_secs(10)).map(|_| Message::CheckOllama);

        Subscription::batch([telemetry_tick, ollama_check])
    }
}

// Async helper functions

async fn read_telemetry_file(path: PathBuf) -> Result<Telemetry, String> {
    let content = tokio::fs::read_to_string(&path)
        .await
        .map_err(|e| e.to_string())?;

    serde_json::from_str(&content).map_err(|e| e.to_string())
}

async fn check_ollama_available() -> bool {
    reqwest::get("http://localhost:11434/api/tags")
        .await
        .map(|r| r.status().is_success())
        .unwrap_or(false)
}
