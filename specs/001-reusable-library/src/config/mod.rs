//! Configuration management for the reusable library framework
//!
//! This module provides configuration management with support for multiple
//! configuration sources and types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Configuration properties
    pub properties: HashMap<String, String>,
}

impl Config {
    /// Create a new configuration
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    /// Set a configuration property
    pub fn set(&mut self, key: &str, value: &str) {
        self.properties.insert(key.to_string(), value.to_string());
    }

    /// Get a configuration property
    pub fn get(&self, key: &str) -> Option<&String> {
        self.properties.get(key)
    }
}

/// Configuration manager trait
pub trait ConfigManager {
    /// Load configuration from source
    fn load(&self) -> Result<Config, Box<dyn std::error::Error>>;

    /// Save configuration to source
    fn save(&self, config: &Config) -> Result<(), Box<dyn std::error::Error>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_creation() {
        let config = Config::new();
        assert_eq!(config.properties.len(), 0);
    }

    #[test]
    fn test_config_set_and_get() {
        let mut config = Config::new();
        config.set("test_key", "test_value");

        assert_eq!(config.get("test_key"), Some(&"test_value".to_string()));
    }
}