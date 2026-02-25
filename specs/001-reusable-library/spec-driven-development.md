# Specification-Driven Development Guide

## Overview

This document provides comprehensive guidance for specification-driven development of the reusable library framework. It ensures that other projects can easily understand, implement, and use this library following clear specifications and best practices.

## Framework Architecture

### Core Concepts

The framework follows a Spring Boot style modular architecture with the following key concepts:

1. **Modularity**: Self-contained modules that provide specific functionality
2. **Dependency Injection**: Container-managed component instantiation and injection
3. **Protocol Abstraction**: Unified interface for different communication protocols
4. **Configuration Management**: Environment-specific settings with validation
5. **Endpoint Exposure**: Client-side discovery of available server endpoints

### Module Structure

Each module follows this standard structure:

```rust
// Module definition
pub struct MyModule {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub config: ModuleConfig,
    pub enabled: bool,
    pub module_type: ModuleType,
}

// Module implementation
impl Module for MyModule {
    fn id(&self) -> &str { &self.id }
    fn name(&self) -> &str { &self.name }
    fn version(&self) -> &str { &self.version }
    fn description(&self) -> &str { &self.description }
    fn dependencies(&self) -> &[String] { &self.dependencies }
    fn initialize(&mut self, config: &ModuleConfig) -> Result<(), Box<dyn std::error::Error>> { /* implementation */ }
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> { /* implementation */ }
    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> { /* implementation */ }
    fn is_running(&self) -> bool { /* implementation */ }
    fn config(&self) -> &ModuleConfig { &self.config }
    fn set_config(&mut self, config: ModuleConfig) { self.config = config; }
}
```

## Protocol Support

### Supported Protocols

The framework supports three main protocols with standardized interfaces:

1. **API Protocol** (RESTful)
2. **gRPC Protocol** (Protocol Buffers)
3. **MCP Protocol** (Model Control Protocol)

### Protocol Interface

All protocols implement the standard Protocol contract:

```rust
pub trait Protocol {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn config(&self) -> &ProtocolConfig;
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn is_running(&self) -> bool;
    fn status(&self) -> ProtocolStatus;
    fn register_service(&mut self, service: &dyn Service) -> Result<(), Box<dyn std::error::Error>>;
    fn unregister_service(&mut self, service_id: &str) -> Result<(), Box<dyn std::error::Error>>;
    fn endpoints(&self) -> &[EndpointInfo]; // NEW: Endpoint exposure
}
```

## Endpoint Exposure

### Client Discovery

Clients can discover available endpoints through the Protocol interface:

```rust
// Get all endpoints
let all_endpoints = protocol.endpoints();

// Get endpoints by protocol type
let api_endpoints = protocol.endpoints_by_protocol("API");

// Get specific endpoint
let endpoint = protocol.endpoint_by_id("endpoint-id");

// Get endpoints by environment
let env_endpoints = protocol.endpoints_by_environment("production");
```

### Endpoint Information

Each endpoint contains comprehensive metadata:

```rust
pub struct EndpointInfo {
    pub id: String,
    pub name: String,
    pub method: String,
    pub path: String,
    pub endpoint_type: EndpointType,
    pub description: String,
    pub input: Option<EndpointParameter>,
    pub output: Option<EndpointParameter>,
    pub status: EndpointStatus,
    pub protocol: String,
    pub environments: Vec<String>,
    pub security: Option<SecurityRequirement>,
}
```

## Configuration Management

### Standard Configuration Structure

```rust
pub struct ModuleConfig {
    pub settings: std::collections::HashMap<String, serde_json::Value>,
    pub environments: std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>,
    pub features: FeatureFlags,
}
```

### Environment-Specific Configuration

Configuration can be overridden per environment:

```toml
# config.toml
[api]
port = 8080
host = "0.0.0.0"
enabled = true

[grpc]
port = 50051
host = "0.0.0.0"
enabled = true

[mcp]
port = 8081
host = "0.0.0.0"
enabled = true

[shared]
log_level = "info"
```

## Dependency Injection

### Container Interface

```rust
pub trait DiContainer {
    fn register_component<T: 'static>(&mut self, id: &str, component: T, scope: Scope) -> Result<(), Box<dyn std::error::Error>>;
    fn get_component<T: 'static>(&self, id: &str) -> Option<&T>;
    fn resolve_dependencies(&self, component_id: &str) -> Result<Vec<Box<dyn Any>>, Box<dyn std::error::Error>>;
    fn inject_dependencies(&self, component: &mut dyn Component) -> Result<(), Box<dyn std::error::Error>>;
    fn config(&self) -> &ContainerConfig;
    fn set_config(&mut self, config: ContainerConfig);
}
```

### Component Registration

```rust
// Register a component with singleton scope
container.register_component::<UserService>("user_service", UserService::new(), Scope::Singleton);

// Get a component instance
let user_service = container.get_component::<UserService>("user_service");
```

## Testing Framework

### Test Utilities

```rust
pub trait TestUtilities {
    fn create_mock_component(&self, component_type: &str) -> Box<dyn Component>;
    fn create_test_config(&self, module_type: &str) -> Config;
    fn run_integration_test(&self, test_case: &TestCase) -> Result<TestResult, Box<dyn std::error::Error>>;
    fn mock_protocol(&self, protocol_type: &str) -> Box<dyn Protocol>;
    fn get_endpoint_info(&self, protocol_id: &str) -> Result<Vec<EndpointInfo>, Box<dyn std::error::Error>>;
}
```

## Implementation Best Practices

### 1. Module Development

When creating new modules:

1. Implement the Module trait
2. Define clear dependencies
3. Provide comprehensive configuration options
4. Expose endpoints through the protocol interface
5. Include proper error handling

### 2. Protocol Implementation

When implementing protocols:

1. Implement the Protocol trait
2. Register endpoints with the protocol
3. Handle protocol-specific configuration
4. Expose endpoint information for client discovery
5. Implement proper error handling and logging

### 3. Service Development

When creating services:

1. Implement the Service trait
2. Define clear service interfaces
3. Handle service lifecycle (initialize, start, stop)
4. Implement proper error handling
5. Provide clear documentation of service operations

## Usage Examples

### Basic Framework Setup

```rust
// Create configuration
let mut config = ConfigManager::new();
config.load_config("config.toml")?;

// Create DI container
let mut container = DiContainer::new();

// Register components
container.register_component::<UserService>("user_service", UserService::new(), Scope::Singleton);
container.register_component::<AuthService>("auth_service", AuthService::new(), Scope::Singleton);

// Create modules
let api_module = ApiModule::new(config.api_config());
let grpc_module = GrpcModule::new(config.grpc_config());
let mcp_module = McpModule::new(config.mcp_config());

// Initialize modules
api_module.initialize(&config)?;
grpc_module.initialize(&config)?;
mcp_module.initialize(&config)?;

// Start modules
api_module.start()?;
grpc_module.start()?;
mcp_module.start()?;
```

### Client Endpoint Discovery

```rust
// Discover all endpoints
let all_endpoints = protocol.endpoints();
for endpoint in all_endpoints {
    println!("Endpoint: {} - {} {}", endpoint.name, endpoint.method, endpoint.path);

    // Check security requirements
    if let Some(security) = &endpoint.security {
        println!("Security: {} with scopes {:?}", security.security_type, security.scopes);
    }
}

// Filter endpoints by environment
let prod_endpoints = protocol.endpoints_by_environment("production");
```

## Versioning and Compatibility

### Semantic Versioning

The framework follows semantic versioning:

- **Major**: Breaking changes
- **Minor**: New features, backward compatible
- **Patch**: Bug fixes, backward compatible

### Backward Compatibility

All contracts are designed to maintain backward compatibility where possible:

1. New methods can be added to existing traits with default implementations
2. Optional parameters are used where appropriate
3. Configuration options are additive
4. Error handling maintains consistent patterns

## Migration Guide

### From Previous Versions

If migrating from previous versions:

1. Update module implementations to follow new trait interfaces
2. Register components with the DI container
3. Update configuration files to new structure
4. Implement endpoint exposure in protocols
5. Update tests to use new testing utilities

## Troubleshooting

### Common Issues

1. **Component Not Found**: Verify component registration in DI container
2. **Configuration Errors**: Check configuration files and validation rules
3. **Protocol Conflicts**: Ensure different protocols use different ports
4. **Endpoint Discovery Issues**: Verify endpoint registration in protocols

### Debugging

Use the framework's logging capabilities to debug issues:

```rust
// Enable detailed logging
let logger = LoggerFactory::get_logger();
logger.set_level(LogLevel::Debug);
```

## Future Roadmap

### Planned Features

1. **Enhanced Security**: Advanced authentication and authorization
2. **Monitoring**: Comprehensive metrics and health checks
3. **Caching**: Optimized caching strategies
4. **Event Streaming**: Real-time event processing
5. **Multi-tenancy**: Support for multi-tenant applications

## Contributing Guidelines

### Code Standards

1. Follow Rust best practices and idioms
2. Include comprehensive documentation
3. Write unit and integration tests
4. Maintain backward compatibility
5. Follow semantic versioning

### Pull Request Process

1. Fork the repository
2. Create feature branch
3. Implement changes with tests
4. Update documentation
5. Submit pull request with clear description

## Support and Community

### Resources

- GitHub Issues: Report bugs and request features
- Documentation: Comprehensive guides and examples
- Community: Join discussions and share knowledge

### Contact

For support or questions, please open an issue on the GitHub repository or contact the maintainers.