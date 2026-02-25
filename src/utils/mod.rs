//! Utility functions and traits for the reusable library framework.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Utility trait for logging
pub trait Logger {
    /// Log info message
    fn info(&self, message: &str);

    /// Log error message
    fn error(&self, message: &str);

    /// Log debug message
    fn debug(&self, message: &str);

    /// Log warning message
    fn warn(&self, message: &str);
}

/// Utility trait for serialization
pub trait Serializer {
    /// Serialize to JSON
    fn to_json(&self) -> Result<String, Box<dyn std::error::Error>>;

    /// Deserialize from JSON
    fn from_json(json: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
}

/// Utility functions for configuration
pub mod config_utils {
    use super::*;

    /// Merge configuration maps
    pub fn merge_config(
        base: &mut HashMap<String, serde_json::Value>,
        override_config: &HashMap<String, serde_json::Value>,
    ) {
        for (key, value) in override_config {
            base.insert(key.clone(), value.clone());
        }
    }

    /// Validate configuration against schema
    pub fn validate_config(
        config: &HashMap<String, serde_json::Value>,
        schema: &HashMap<String, String>,
    ) -> bool {
        // Simple validation logic
        for (key, expected_type) in schema {
            if let Some(value) = config.get(key) {
                let actual_type = value.as_str().map(|_| "string")
                    .or_else(|| value.as_i64().map(|_| "integer"))
                    .or_else(|| value.as_f64().map(|_| "float"))
                    .or_else(|| value.as_bool().map(|_| "boolean"))
                    .unwrap_or("unknown");

                if actual_type != *expected_type {
                    return false;
                }
            }
        }
        true
    }
}

/// Utility functions for modules
pub mod module_utils {
    use super::*;

    /// Validate module dependencies
    pub fn validate_dependencies(
        dependencies: &[String],
        available_modules: &[String],
    ) -> bool {
        for dep in dependencies {
            if !available_modules.contains(dep) {
                return false;
            }
        }
        true
    }

    /// Generate module ID
    pub fn generate_module_id(name: &str, version: &str) -> String {
        format!("{}-{}", name, version)
    }
}

/// Utility functions for components
pub mod component_utils {
    use super::*;

    /// Resolve component dependencies
    pub fn resolve_dependencies(
        _dependencies: &[String],
        _container: &dyn crate::di::DiContainer,
    ) -> Vec<Box<dyn std::any::Any>> {
        // Placeholder for dependency resolution
        vec![]
    }
}

pub mod tests;