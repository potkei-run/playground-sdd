//! Dependency injection container for the reusable library framework.

use std::any::Any;
use serde::{Deserialize, Serialize};
use anyhow::Result;

/// Dependency injection container trait
pub trait DiContainer {
    /// Register component with container
    fn register_component(
        &mut self,
        id: &str,
        component: Box<dyn Any>,
        scope: Scope,
    ) -> Result<(), Box<dyn std::error::Error>>;

    /// Get component instance from container
    fn get_component(&self, id: &str) -> Option<&dyn Any>;

    /// Resolve dependencies for component
    fn resolve_dependencies(
        &self,
        component_id: &str,
    ) -> Result<Vec<Box<dyn Any>>, Box<dyn std::error::Error>>;

    /// Inject dependencies into component
    fn inject_dependencies(
        &self,
        component: &mut dyn Any,
    ) -> Result<(), Box<dyn std::error::Error>>;

    /// Get container configuration
    fn config(&self) -> &ContainerConfig;

    /// Set container configuration
    fn set_config(&mut self, config: ContainerConfig);
}

/// Component scope enumeration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum Scope {
    /// Transient - new instance each time
    #[default]
    Transient,

    /// Singleton - single instance for entire application
    Singleton,

    /// Scoped - instance per scope (request, session, etc.)
    Scoped,
}

/// Container configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContainerConfig {
    /// Auto-scan for components
    pub auto_scan: bool,

    /// Scan paths for components
    pub scan_paths: Vec<String>,

    /// Default scope for components
    pub default_scope: Scope,

    /// Enable caching
    pub enable_caching: bool,
}

/// Component trait
pub trait Component {
    /// Get component identifier
    fn id(&self) -> &str;

    /// Get component type
    fn component_type(&self) -> &str;

    /// Get component scope
    fn scope(&self) -> Scope;

    /// Get component dependencies
    fn dependencies(&self) -> &[String];

    /// Initialize component
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Get component instance
    fn instance(&self) -> &dyn Any;
}

/// Component implementation
#[derive(Debug)]
pub struct ComponentImpl {
    /// Component identifier
    pub id: String,

    /// Component type
    pub component_type: String,

    /// Component scope
    pub scope: Scope,

    /// Component dependencies
    pub dependencies: Vec<String>,

    /// Component instance
    pub instance: Box<dyn Any>,
}

impl Component for ComponentImpl {
    fn id(&self) -> &str {
        &self.id
    }

    fn component_type(&self) -> &str {
        &self.component_type
    }

    fn scope(&self) -> Scope {
        self.scope.clone()
    }

    fn dependencies(&self) -> &[String] {
        &self.dependencies
    }

    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn instance(&self) -> &dyn Any {
        self.instance.as_ref()
    }
}

pub mod container;
pub use container::DiContainerImpl;