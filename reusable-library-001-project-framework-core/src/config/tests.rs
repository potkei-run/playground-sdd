//! Tests for the configuration manager.

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_config_manager_creation() {
        let config_manager = ConfigManagerImpl {
            base_config: HashMap::new(),
            environments: HashMap::new(),
            sources: vec![],
            validation: ValidationRules {
                required: vec![],
                patterns: HashMap::new(),
                types: HashMap::new(),
            },
        };

        assert!(config_manager.base_config.is_empty());
        assert!(config_manager.environments.is_empty());
        assert!(config_manager.sources.is_empty());
    }

    #[test]
    fn test_config_structure() {
        let config = Config {
            settings: HashMap::new(),
            environment: "test".to_string(),
            version: "1.0.0".to_string(),
            timestamp: std::time::SystemTime::now(),
        };

        assert_eq!(config.environment, "test");
        assert_eq!(config.version, "1.0.0");
    }
}