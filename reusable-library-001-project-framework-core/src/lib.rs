//! Core framework implementation for the project framework update.
//!
//! This module provides the foundational components for the framework, including:
//! - Dependency injection container
//! - Configuration management
//! - Module lifecycle management
//! - Core utilities and helpers

#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]

pub mod config;
pub mod di;
pub mod modules;
pub mod utils;

/// Main framework entry point
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize core framework components
    tracing::info!("Initializing reusable-library-001-project-framework-core");

    // Initialize dependency injection container
    // Note: The container doesn't have an init function in this implementation
    // We'll just create a new instance instead
    let container_config = crate::di::ContainerConfig::default();
    let _container = crate::di::DiContainerImpl::new(container_config);

    // Initialize configuration
    // Note: The config module doesn't have an init function in this implementation
    // We'll just call the init function we added
    crate::config::init();

    tracing::info!("Framework initialized successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_framework_init() {
        assert!(init().is_ok());
    }
}