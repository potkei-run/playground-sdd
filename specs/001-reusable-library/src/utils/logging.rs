//! Logging and monitoring capabilities
//!
//! This module provides structured logging and monitoring for the reusable library framework.

use tracing::{info, warn, error, debug};
use tracing_subscriber;

/// Initialize the logging system
pub fn init_logging() {
    tracing_subscriber::fmt::init();
    info!("Logging system initialized");
}

/// Log an info message
pub fn log_info(message: &str) {
    info!("{}", message);
}

/// Log a warning message
pub fn log_warn(message: &str) {
    warn!("{}", message);
}

/// Log an error message
pub fn log_error(message: &str) {
    error!("{}", message);
}

/// Log a debug message
pub fn log_debug(message: &str) {
    debug!("{}", message);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logging_initialization() {
        // This test just verifies the function compiles and runs
        init_logging();
        log_info("Test info message");
        log_warn("Test warning message");
        log_error("Test error message");
        log_debug("Test debug message");

        assert_eq!(1, 1);
    }
}