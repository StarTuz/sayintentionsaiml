//! Stratus ATC - GUI Application
//!
//! Iced-based desktop application with embedded Axum web server for ComLink.

mod app;
mod comlink;
mod theme;

use anyhow::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tracing::info!("Stratus ATC starting...");

    // Run the Iced application
    iced::application(
        "Stratus ATC",
        app::StratusApp::update,
        app::StratusApp::view,
    )
    .theme(app::StratusApp::theme)
    .subscription(app::StratusApp::subscription)
    .run_with(app::StratusApp::new)?;

    Ok(())
}
