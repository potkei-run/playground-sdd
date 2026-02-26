//! Tests for the library module implementation.

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_library_module_creation() {
        let module = LibraryModule::new(
            "test-module".to_string(),
            "Test Module".to_string(),
            "1.0.0".to_string(),
            "A test module".to_string(),
            vec!["dependency1".to_string(), "dependency2".to_string()],
            ModuleType::Core,
        );

        assert_eq!(module.id(), "test-module");
        assert_eq!(module.name(), "Test Module");
        assert_eq!(module.version(), "1.0.0");
        assert_eq!(module.description(), "A test module");
        assert_eq!(module.dependencies().len(), 2);
        assert!(module.is_running() == false);
    }

    #[test]
    fn test_module_status() {
        let mut module = LibraryModule::new(
            "test-module".to_string(),
            "Test Module".to_string(),
            "1.0.0".to_string(),
            "A test module".to_string(),
            vec![],
            ModuleType::Core,
        );

        assert_eq!(module.status(), &ModuleStatus::Disabled);

        // Initialize module
        let config = ModuleConfig {
            settings: HashMap::new(),
            environments: HashMap::new(),
            features: FeatureFlags {
                enabled: true,
                version: "1.0.0".to_string(),
                features: HashMap::new(),
                restrictions: HashMap::new(),
            },
        };

        let result = module.initialize(&config);
        assert!(result.is_ok());
        assert_eq!(module.status(), &ModuleStatus::Initializing);

        // Start module
        let result = module.start();
        assert!(result.is_ok());
        assert_eq!(module.status(), &ModuleStatus::Ready);
    }
}