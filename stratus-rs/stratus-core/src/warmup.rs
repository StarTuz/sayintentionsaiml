//! Model Warmup Service
//!
//! Keeps the Ollama model hot by sending periodic lightweight prompts.
//! Eliminates cold-start latency (5-15s) when model hasn't been used recently.

use reqwest::Client;
use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::watch;
use tokio::time;
use tracing::{debug, info, warn};

/// Warmup service configuration
#[derive(Debug, Clone)]
pub struct WarmupConfig {
    pub model: String,
    pub interval: Duration,
    pub ollama_url: String,
}

impl Default for WarmupConfig {
    fn default() -> Self {
        Self {
            model: "llama3.2:3b".to_string(),
            interval: Duration::from_secs(30),
            ollama_url: "http://localhost:11434".to_string(),
        }
    }
}

/// Request for minimal warmup prompt
#[derive(Debug, Serialize)]
struct WarmupRequest {
    model: String,
    prompt: String,
    stream: bool,
    options: WarmupOptions,
}

#[derive(Debug, Serialize)]
struct WarmupOptions {
    num_predict: i32,
    temperature: f32,
}

/// Statistics from the warmup service
#[derive(Debug, Clone, Default)]
pub struct WarmupStats {
    pub heartbeat_count: u64,
    pub last_latency_ms: u64,
    pub is_running: bool,
    pub is_paused: bool,
}

/// Model warmup service handle
pub struct WarmupService {
    config: WarmupConfig,
    running: Arc<AtomicBool>,
    paused: Arc<AtomicBool>,
    stats_tx: watch::Sender<WarmupStats>,
    stats_rx: watch::Receiver<WarmupStats>,
}

impl WarmupService {
    /// Create a new warmup service
    pub fn new(config: WarmupConfig) -> Self {
        let (stats_tx, stats_rx) = watch::channel(WarmupStats::default());

        Self {
            config,
            running: Arc::new(AtomicBool::new(false)),
            paused: Arc::new(AtomicBool::new(false)),
            stats_tx,
            stats_rx,
        }
    }

    /// Start the warmup service
    pub fn start(&self) {
        if self.running.load(Ordering::SeqCst) {
            warn!("Warmup service already running");
            return;
        }

        self.running.store(true, Ordering::SeqCst);

        let config = self.config.clone();
        let running = self.running.clone();
        let paused = self.paused.clone();
        let stats_tx = self.stats_tx.clone();

        tokio::spawn(async move {
            info!(
                "Warmup service started: model={}, interval={}s",
                config.model,
                config.interval.as_secs()
            );

            let client = Client::new();
            let mut heartbeat_count = 0u64;

            while running.load(Ordering::SeqCst) {
                // Wait for interval
                time::sleep(config.interval).await;

                // Skip if paused
                if paused.load(Ordering::SeqCst) {
                    debug!("Warmup paused, skipping heartbeat");
                    continue;
                }

                // Send heartbeat
                let latency = send_heartbeat(&client, &config).await;
                heartbeat_count += 1;

                let stats = WarmupStats {
                    heartbeat_count,
                    last_latency_ms: latency,
                    is_running: true,
                    is_paused: paused.load(Ordering::SeqCst),
                };

                let _ = stats_tx.send(stats);
                debug!("Warmup heartbeat: {}ms", latency);
            }

            info!("Warmup service stopped");
        });
    }

    /// Stop the warmup service
    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Pause heartbeats (e.g., when PTT is active)
    pub fn pause(&self) {
        self.paused.store(true, Ordering::SeqCst);
        debug!("Warmup service paused");
    }

    /// Resume heartbeats
    pub fn resume(&self) {
        self.paused.store(false, Ordering::SeqCst);
        debug!("Warmup service resumed");
    }

    /// Force an immediate warmup (useful before expected use)
    pub async fn force_warmup(&self) -> u64 {
        let client = Client::new();
        let latency = send_heartbeat(&client, &self.config).await;
        info!("Forced warmup complete: {}ms", latency);
        latency
    }

    /// Subscribe to stats updates
    pub fn stats(&self) -> watch::Receiver<WarmupStats> {
        self.stats_rx.clone()
    }

    /// Check if service is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Check if service is paused
    pub fn is_paused(&self) -> bool {
        self.paused.load(Ordering::SeqCst)
    }
}

/// Send a minimal warmup prompt to keep model loaded
async fn send_heartbeat(client: &Client, config: &WarmupConfig) -> u64 {
    let start = Instant::now();

    let request = WarmupRequest {
        model: config.model.clone(),
        prompt: "Ready".to_string(),
        stream: false,
        options: WarmupOptions {
            num_predict: 5,
            temperature: 0.0,
        },
    };

    let result = client
        .post(format!("{}/api/generate", config.ollama_url))
        .json(&request)
        .timeout(Duration::from_secs(15))
        .send()
        .await;

    let latency = start.elapsed().as_millis() as u64;

    match result {
        Ok(response) if response.status().is_success() => {
            debug!("Warmup successful: {}ms", latency);
        }
        Ok(response) => {
            warn!("Warmup failed with status: {}", response.status());
        }
        Err(e) => {
            warn!("Warmup request failed: {}", e);
        }
    }

    latency
}

impl Default for WarmupService {
    fn default() -> Self {
        Self::new(WarmupConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_warmup_config_default() {
        let config = WarmupConfig::default();
        assert_eq!(config.interval, Duration::from_secs(30));
        assert_eq!(config.model, "llama3.2:3b");
    }
}
