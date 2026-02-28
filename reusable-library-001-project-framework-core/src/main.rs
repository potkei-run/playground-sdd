//! Main entry point for the reusable-library-001-project-framework-core
//!
//! This binary initializes and runs the core framework components.

use reusable_library_001_project_framework_core::init;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    init()?;

    tracing::info!("Core framework running");

    // Keep the application running
    tokio::signal::ctrl_c().await?;

    tracing::info!("Shutting down framework");
    Ok(())
}