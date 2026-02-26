//! Dependency injection container implementation for the reusable library framework.

use crate::di::*;
use std::collections::HashMap;
use std::any::Any;
use anyhow::Result;

/// Dependency injection container implementation
#[derive(Debug)]
pub struct DiContainerImpl {
    /// Registered components
    pub components: HashMap<String, ComponentImpl>,

    /// Component instances (singleton scope)
    pub instances: HashMap<String, Box<dyn Any>>,

    /// Configuration for the container
    pub config: ContainerConfig,
}

impl DiContainerImpl {
    /// Create a new dependency injection container
    pub fn new(config: ContainerConfig) -> Self {
        Self {
            components: HashMap::new(),
            instances: HashMap::new(),
            config,
        }
    }

    /// Get component instance from container
    pub fn get_component_instance<T: 'static>(&self, id: &str) -> Option<&T> {
        // Try to get from instances first (singleton)
        if let Some(instance) = self.instances.get(id) {
            return instance.downcast_ref::<T>();
        }

        // Try to get from components
        if let Some(component) = self.components.get(id) {
            if let Some(instance) = component.instance.downcast_ref::<T>() {
                return Some(instance);
            }
        }

        None
    }

    /// Register a component with the container
    pub fn register_component<T: 'static>(
        &mut self,
        id: &str,
        component: T,
        scope: Scope,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let component_impl = ComponentImpl {
            id: id.to_string(),
            component_type: std::any::type_name::<T>().to_string(),
            scope,
            dependencies: Vec::new(),
            instance: Box::new(component),
        };

        self.components.insert(id.to_string(), component_impl);
        Ok(())
    }

    /// Resolve dependencies for a component
    pub fn resolve_dependencies(
        &self,
        component_id: &str,
    ) -> Result<Vec<Box<dyn Any>>, Box<dyn std::error::Error>> {
        let mut dependencies = Vec::new();

        // In a real implementation, this would resolve actual dependencies
        // For now, return empty vector as placeholder
        Ok(dependencies)
    }

    /// Inject dependencies into component
    pub fn inject_dependencies(
        &self,
        component: &mut dyn Any,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // In a real implementation, this would inject dependencies
        // For now, do nothing as placeholder
        Ok(())
    }
}

impl DiContainer for DiContainerImpl {
    /// Register component with container
    fn register_component(
        &mut self,
        id: &str,
        component: Box<dyn Any>,
        scope: Scope,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let component_impl = ComponentImpl {
            id: id.to_string(),
            component_type: format!("{:?}", component.type_id()),
            scope,
            dependencies: Vec::new(),
            instance: component,
        };

        self.components.insert(id.to_string(), component_impl);
        Ok(())
    }

    /// Get component instance from container
    fn get_component(&self, id: &str) -> Option<&dyn Any> {
        // Try to get from instances first (singleton)
        if let Some(instance) = self.instances.get(id) {
            return Some(instance.as_ref());
        }

        // Try to get from components
        if let Some(component) = self.components.get(id) {
            return Some(component.instance.as_ref());
        }

        None
    }

    /// Resolve dependencies for component
    fn resolve_dependencies(
        &self,
        _component_id: &str,
    ) -> Result<Vec<Box<dyn Any>>, Box<dyn std::error::Error>> {
        // In a real implementation, this would resolve actual dependencies
        // For now, return empty vector as placeholder
        Ok(Vec::new())
    }

    /// Inject dependencies into component
    fn inject_dependencies(
        &self,
        _component: &mut dyn Any,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // In a real implementation, this would inject dependencies
        // For now, do nothing as placeholder
        Ok(())
    }

    /// Get container configuration
    fn config(&self) -> &ContainerConfig {
        &self.config
    }

    /// Set container configuration
    fn set_config(&mut self, config: ContainerConfig) {
        self.config = config;
    }
}