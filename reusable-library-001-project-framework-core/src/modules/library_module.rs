//! Library module implementation for the reusable library framework.

use crate::modules::*;
use crate::di::*;
use crate::config::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::Result;

/// Library module implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryModule {
    /// Unique identifier for the module
    pub id: String,

    /// Human-readable name of the module
    pub name: String,

    /// Version of the module
    pub version: String,

    /// Module description and purpose
    pub description: String,

    /// Module dependencies
    pub dependencies: Vec<String>,

    /// Module configuration
    pub config: ModuleConfig,

    /// Module status (enabled/disabled)
    pub enabled: bool,

    /// Module type (API, gRPC, Core, etc.)
    pub module_type: ModuleType,

    /// Module status
    pub status: ModuleStatus,

    /// Module components
    pub components: Vec<String>,
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

impl LibraryModule {
    /// Create a new library module
    pub fn new(
        id: String,
        name: String,
        version: String,
        description: String,
        dependencies: Vec<String>,
        module_type: ModuleType,
    ) -> Self {
        Self {
            id,
            name,
            version,
            description,
            dependencies,
            config: ModuleConfig {
                settings: HashMap::new(),
                environments: HashMap::new(),
                features: FeatureFlags {
                    enabled: true,
                    version: "1.0.0".to_string(),
                    features: HashMap::new(),
                    restrictions: HashMap::new(),
                },
            },
            enabled: true,
            module_type,
            status: ModuleStatus::Disabled,
            components: Vec::new(),
        }
    }

    /// Get module status
    pub fn status(&self) -> &ModuleStatus {
        &self.status
    }

    /// Set module status
    pub fn set_status(&mut self, status: ModuleStatus) {
        self.status = status;
    }
}

impl Module for LibraryModule {
    /// Get module identifier
    fn id(&self) -> &str {
        &self.id
    }

    /// Get module name
    fn name(&self) -> &str {
        &self.name
    }

    /// Get module version
    fn version(&self) -> &str {
        &self.version
    }

    /// Get module description
    fn description(&self) -> &str {
        &self.description
    }

    /// Get module dependencies
    fn dependencies(&self) -> &[String] {
        &self.dependencies
    }

    /// Initialize the module
    fn initialize(&mut self, config: &ModuleConfig) -> Result<(), Box<dyn std::error::Error>> {
        self.config = config.clone();
        self.status = ModuleStatus::Initializing;
        Ok(())
    }

    /// Start the module
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.enabled {
            self.status = ModuleStatus::Ready;
        } else {
            self.status = ModuleStatus::Disabled;
        }
        Ok(())
    }

    /// Stop the module
    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.status = ModuleStatus::Disabled;
        Ok(())
    }

    /// Check if module is running
    fn is_running(&self) -> bool {
        matches!(self.status, ModuleStatus::Ready)
    }

    /// Get module configuration
    fn config(&self) -> &ModuleConfig {
        &self.config
    }

    /// Set module configuration
    fn set_config(&mut self, config: ModuleConfig) {
        self.config = config;
    }
}