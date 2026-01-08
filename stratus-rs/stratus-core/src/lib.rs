//! Stratus Core - ATC Engine Library
//!
//! This crate contains the core logic for Stratus ATC:
//! - Telemetry: File-based communication with X-Plane
//! - Ollama: Local LLM client
//! - Streaming: Low-latency streaming LLM responses
//! - Warmup: Keep model hot to eliminate cold-starts
//! - ATC: Prompt building and response parsing

pub mod atc;
pub mod ollama;
pub mod telemetry;
pub mod streaming;
pub mod warmup;

// Re-export common types
pub use atc::AtcEngine;
pub use ollama::OllamaClient;
pub use telemetry::{TelemetryWatcher, Telemetry};
pub use streaming::{StreamChunk, StreamingOllama};
pub use warmup::{WarmupConfig, WarmupService};
