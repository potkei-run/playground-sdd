//! Reusable library framework for building modular applications.
//!
//! This crate provides a foundation for building reusable, modular applications
//! with dependency injection, configuration management, and module lifecycle management.

// Declare modules
mod modules;
mod di;
mod config;
mod utils;

// Re-export core modules
pub use modules::*;
pub use di::*;
pub use config::*;
pub use utils::*;

// Re-export common types
pub use serde_json::Value;

/// Main library entry point
pub mod prelude {
    pub use crate::modules::*;
    pub use crate::di::*;
    pub use crate::config::*;
    pub use crate::utils::*;
}

/// Library version
pub const VERSION: &str = "0.1.0";