//! Core dependency injection container implementation
//!
//! This module provides the main container for managing dependencies and
//! their lifecycle within the reusable library framework.

use std::collections::HashMap;

/// Dependency injection container
pub struct Container {
    /// Registry of registered dependencies
    registry: HashMap<String, Box<dyn std::any::Any + Send + Sync>>,
}

impl Container {
    /// Create a new container
    pub fn new() -> Self {
        Self {
            registry: HashMap::new(),
        }
    }

    /// Register a dependency
    pub fn register<T: 'static + Send + Sync>(&mut self, name: &str, dependency: T) {
        self.registry.insert(name.to_string(), Box::new(dependency));
    }

    /// Resolve a dependency by name
    pub fn resolve<T: 'static>(&self, name: &str) -> Option<&T> {
        self.registry
            .get(name)
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container_creation() {
        let container = Container::new();
        assert_eq!(container.registry.len(), 0);
    }

    #[test]
    fn test_register_and_resolve() {
        let mut container = Container::new();
        let service = "test_service";
        container.register("test_service", service);

        let resolved = container.resolve::<&str>("test_service");
        assert!(resolved.is_some());
        assert_eq!(resolved.unwrap(), &"test_service");
    }
}