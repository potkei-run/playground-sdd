//! Module management for the framework.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::Result;

/// Module identifier trait
pub trait Module {
    /// Get module identifier
    fn id(&self) -> &str;

    /// Get module name
    fn name(&self) -> &str;

    /// Get module version
    fn version(&self) -> &str;

    /// Get module description
    fn description(&self) -> &str;

    /// Get module dependencies
    fn dependencies(&self) -> &[String];

    /// Initialize the module
    fn initialize(&mut self, config: &ModuleConfig) -> Result<(), Box<dyn std::error::Error>>;

    /// Start the module
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Stop the module
    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Check if module is running
    fn is_running(&self) -> bool;

    /// Get module configuration
    fn config(&self) -> &ModuleConfig;

    /// Set module configuration
    fn set_config(&mut self, config: ModuleConfig);
}

/// Module configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleConfig {
    /// Module-specific settings
    pub settings: HashMap<String, serde_json::Value>,

    /// Environment-specific overrides
    pub environments: HashMap<String, HashMap<String, serde_json::Value>>,

    /// Feature flags for module capabilities
    pub features: FeatureFlags,
}

impl Default for ModuleConfig {
    fn default() -> Self {
        Self {
            settings: HashMap::new(),
            environments: HashMap::new(),
            features: FeatureFlags::default(),
        }
    }
}

/// Feature flags for module capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlags {
    pub enabled: bool,
    pub version: String,
    pub features: HashMap<String, bool>,
    pub restrictions: HashMap<String, FeatureRestriction>,
}

impl Default for FeatureFlags {
    fn default() -> Self {
        Self {
            enabled: true,
            version: "1.0.0".to_string(),
            features: HashMap::new(),
            restrictions: HashMap::new(),
        }
    }
}

/// Feature restriction configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureRestriction {
    pub max_concurrent: Option<u32>,
    pub timeout: Option<std::time::Duration>,
    pub rate_limit: Option<u32>,
}

/// Module type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModuleType {
    /// Core framework module
    Core,

    /// API module
    Api,

    /// gRPC module
    Grpc,

    /// MCP module
    Mcp,

    /// Custom module
    Custom(String),
}

/// Module status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModuleStatus {
    /// Module is disabled
    Disabled,

    /// Module is initializing
    Initializing,

    /// Module is ready
    Ready,

    /// Module is in error state
    Error(String),
}

pub mod library_module;
pub use library_module::LibraryModule;

/// Initialize modules
pub fn init() {
    tracing::info!("Initializing modules");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_config_defaults() {
        let config = ModuleConfig::default();
        assert_eq!(config.features.enabled, true);
        assert_eq!(config.features.version, "1.0.0");
    }

    #[test]
    fn test_feature_flags_defaults() {
        let flags = FeatureFlags::default();
        assert_eq!(flags.enabled, true);
        assert_eq!(flags.version, "1.0.0");
    }
}