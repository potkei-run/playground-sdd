//! Tests for the dependency injection container.

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_di_container_creation() {
        let config = ContainerConfig {
            auto_scan: true,
            scan_paths: vec!["src".to_string()],
            default_scope: Scope::Singleton,
            enable_caching: true,
        };

        let container = DiContainerImpl::new(config);

        assert!(container.components.is_empty());
        assert!(container.instances.is_empty());
    }

    #[test]
    fn test_component_registration() {
        let config = ContainerConfig {
            auto_scan: true,
            scan_paths: vec!["src".to_string()],
            default_scope: Scope::Singleton,
            enable_caching: true,
        };

        let mut container = DiContainerImpl::new(config);

        // Register a simple component
        let result = container.register_component("test_component", "test_value", Scope::Singleton);
        assert!(result.is_ok());
    }
}