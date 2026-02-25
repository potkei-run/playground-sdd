# Contracts: Reusable Library with Spring Boot Style Framework

## Overview

This document defines the contracts for the reusable library framework that supports modular architecture with Spring Boot style dependency injection and multiple protocol support (API, gRPC, MCP).

## Core Contracts

### 1. Module Contract
Defines the interface that all modules must implement.

```rust
pub trait Module {
    /// Get module identifier
    fn id(&self) -> &str;

    /// Get module name
    fn name(&self) -> &str;

    /// Get module version
    fn version(&self -> &str;

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
```

### 2. Component Contract
Defines the interface for injectable components.

```rust
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
```

### 3. Protocol Contract
Defines the interface for communication protocols.

```rust
pub trait Protocol {
    /// Get protocol identifier
    fn id(&self) -> &str;

    /// Get protocol name
    fn name(&self) -> &str;

    /// Get protocol configuration
    fn config(&self) -> &ProtocolConfig;

    /// Initialize protocol
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Start protocol
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Stop protocol
    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Check if protocol is running
    fn is_running(&self) -> bool;

    /// Get protocol status
    fn status(&self) -> ProtocolStatus;

    /// Register service with protocol
    fn register_service(&mut self, service: &dyn Service) -> Result<(), Box<dyn std::error::Error>>;

    /// Unregister service from protocol
    fn unregister_service(&mut self, service_id: &str) -> Result<(), Box<dyn std::error::Error>>;

    /// Get available endpoints for client discovery
    fn endpoints(&self) -> &[EndpointInfo];
}
```

### 4. Service Contract
Defines the interface for business logic services.

```rust
pub trait Service {
    /// Get service identifier
    fn id(&self) -> &str;

    /// Get service name
    fn name(&self) -> &str;

    /// Get service description
    fn description(&self) -> &str;

    /// Get service dependencies
    fn dependencies(&self) -> &[String];

    /// Initialize service
    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Start service
    fn start(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Stop service
    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    /// Check if service is running
    fn is_running(&self) -> bool;

    /// Execute service operation
    fn execute(&self, request: &Request) -> Result<Response, Box<dyn std::error::Error>>;
}
```

### 5. Dependency Injection Container Contract
Defines the interface for the DI container.

```rust
pub trait DiContainer {
    /// Register component with container
    fn register_component<T: 'static>(&mut self, id: &str, component: T, scope: Scope) -> Result<(), Box<dyn std::error::Error>>;

    /// Get component instance from container
    fn get_component<T: 'static>(&self, id: &str) -> Option<&T>;

    /// Resolve dependencies for component
    fn resolve_dependencies(&self, component_id: &str) -> Result<Vec<Box<dyn Any>>, Box<dyn std::error::Error>>;

    /// Inject dependencies into component
    fn inject_dependencies(&self, component: &mut dyn Component) -> Result<(), Box<dyn std::error::Error>>;

    /// Get container configuration
    fn config(&self) -> &ContainerConfig;

    /// Set container configuration
    fn set_config(&mut self, config: ContainerConfig);
}
```

### 6. Configuration Manager Contract
Defines the interface for configuration management.

```rust
pub trait ConfigManager {
    /// Load configuration from source
    fn load_config(&mut self, source: &str) -> Result<Config, Box<dyn std::error::Error>>;

    /// Get configuration value
    fn get_value(&self, key: &str) -> Option<&serde_json::Value>;

    /// Set configuration value
    fn set_value(&mut self, key: &str, value: serde_json::Value);

    /// Get environment configuration
    fn get_env_config(&self, environment: &str) -> Result<Config, Box<dyn std::error::Error>>;

    /// Validate configuration
    fn validate_config(&self, config: &Config) -> Result<(), Box<dyn std::error::Error>>;

    /// Apply configuration to module
    fn apply_to_module(&self, module: &mut dyn Module) -> Result<(), Box<dyn std::error::Error>>;
}
```

## Protocol-Specific Contracts

### 7. API Protocol Contract
```rust
pub trait ApiProtocol: Protocol {
    /// Register API endpoint
    fn register_endpoint(&mut self, endpoint: ApiEndpoint) -> Result<(), Box<dyn std::error::Error>>;

    /// Get registered endpoints
    fn endpoints(&self) -> &[ApiEndpoint];

    /// Handle HTTP request
    fn handle_request(&self, request: &HttpRequest) -> Result<HttpResponse, Box<dyn std::error::Error>>;
}
```

### 8. gRPC Protocol Contract
```rust
pub trait GrpcProtocol: Protocol {
    /// Register gRPC service
    fn register_service(&mut self, service: GrpcService) -> Result<(), Box<dyn std::error::Error>>;

    /// Get registered services
    fn services(&self) -> &[GrpcService];

    /// Handle gRPC request
    fn handle_request(&self, request: &GrpcRequest) -> Result<GrpcResponse, Box<dyn std::error::Error>>;
}
```

### 9. MCP Protocol Contract
```rust
pub trait McpProtocol: Protocol {
    /// Register MCP handler
    fn register_handler(&mut self, handler: McpHandler) -> Result<(), Box<dyn std::error::Error>>;

    /// Get registered handlers
    fn handlers(&self) -> &[McpHandler];

    /// Handle MCP message
    fn handle_message(&self, message: &McpMessage) -> Result<McpResponse, Box<dyn std::error::Error>>;
}
````

## Data Contracts

### 10. Request/Response Contracts
```rust
pub struct Request {
    pub id: String,
    pub method: String,
    pub path: String,
    pub headers: std::collections::HashMap<String, String>,
    pub body: serde_json::Value,
    pub timestamp: std::time::SystemTime,
}

pub struct Response {
    pub id: String,
    pub status: u16,
    pub headers: std::collections::HashMap<String, String>,
    pub body: serde_json::Value,
    pub timestamp: std::time::SystemTime,
}
````

### 11. Configuration Contracts
```rust
pub struct Config {
    pub settings: std::collections::HashMap<String, serde_json::Value>,
    pub environment: String,
    pub version: String,
    pub timestamp: std::time::SystemTime,
}

pub struct ContainerConfig {
    pub auto_scan: bool,
    pub scan_paths: Vec<String>,
    pub default_scope: Scope,
    pub enable_caching: bool,
}
````

### 12. Endpoint Information Contract
```rust
pub struct EndpointInfo {
    /// Endpoint identifier
    pub id: String,

    /// Endpoint name
    pub name: String,

    /// HTTP method or gRPC method
    pub method: String,

    /// Endpoint path or gRPC service/method
    pub path: String,

    /// Endpoint type (HTTP, gRPC, WebSocket)
    pub endpoint_type: EndpointType,

    /// Description of the endpoint
    pub description: String,

    /// Input parameters
    pub input: Option<EndpointParameter>,

    /// Output parameters
    pub output: Option<EndpointParameter>,

    /// Endpoint status
    pub status: EndpointStatus,

    /// Protocol this endpoint belongs to
    pub protocol: String,

    /// Available in which environments
    pub environments: Vec<String>,

    /// Security requirements
    pub security: Option<SecurityRequirement>,
}
````

### 13. Endpoint Parameter Contract
```rust
pub struct EndpointParameter {
    /// Parameter name
    pub name: String,

    /// Parameter type
    pub param_type: String,

    /// Whether parameter is required
    pub required: bool,

    /// Parameter description
    pub description: String,

    /// Default value
    pub default_value: Option<serde_json::Value>,

    /// Validation rules
    pub validation: Option<ValidationRule>,
}
````

### 14. Endpoint Type Contract
```rust
pub enum EndpointType {
    /// HTTP endpoint
    Http,

    /// gRPC endpoint
    Grpc,

    /// WebSocket endpoint
    WebSocket,

    /// Event endpoint
    Event,
}
````

### 15. Endpoint Status Contract
```rust
pub enum EndpointStatus {
    /// Endpoint is active and available
    Active,

    /// Endpoint is deprecated
    Deprecated,

    /// Endpoint is disabled
    Disabled,

    /// Endpoint is in maintenance
    Maintenance,
}
````

### 16. Security Requirement Contract
```rust
pub struct SecurityRequirement {
    /// Security type (JWT, OAuth, API Key, etc.)
    pub security_type: String,

    /// Required scopes
    pub scopes: Vec<String>,

    /// Required roles
    pub roles: Vec<String>,

    /// Authentication method
    pub auth_method: String,
}
````

## Error Contracts

### 17. Framework Error Contract
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum FrameworkError {
    /// Module not found
    ModuleNotFound(String),

    /// Component not found
    ComponentNotFound(String),

    /// Dependency resolution error
    DependencyError(String),

    /// Configuration error
    ConfigError(String),

    /// Protocol error
    ProtocolError(String),

    /// Service error
    ServiceError(String),

    /// Endpoint discovery error
    EndpointError(String),

    /// Generic framework error
    Generic(String),
}
````

## Feature Contracts

### 18. Feature Flags Contract
```rust
pub struct FeatureFlags {
    pub enabled: bool,
    pub version: String,
    pub features: std::collections::HashMap<String, bool>,
    pub restrictions: std::collections::HashMap<String, FeatureRestriction>,
}

pub struct FeatureRestriction {
    pub max_concurrent: Option<u32>,
    pub timeout: Option<std::time::Duration>,
    pub rate_limit: Option<u32>,
}
````

## Versioning Contract

### 19. Semantic Versioning Contract
```rust
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre_release: Option<String>,
    pub build_metadata: Option<String>,
}

impl Version {
    /// Parse version string
    pub fn parse(version: &str) -> Result<Version, Box<dyn std::error::Error>>;

    /// Compare versions
    pub fn compare(&self, other: &Version) -> std::cmp::Ordering;

    /// Check if version is compatible
    pub fn is_compatible(&self, other: &Version) -> bool;
}
````

## Integration Contracts

### 20. Module Integration Contract
```rust
pub trait ModuleIntegration {
    /// Get integration points
    fn integration_points(&self) -> &[IntegrationPoint];

    /// Register integration point
    fn register_integration_point(&mut self, point: IntegrationPoint) -> Result<(), Box<dyn std::error::Error>>;

    /// Execute integration
    fn execute_integration(&self, point: &IntegrationPoint, data: &serde_json::Value) -> Result<serde_json::Value, Box<dyn std::error::Error>>;
}
````

## Testing Contracts

### 21. Test Utilities Contract
```rust
pub trait TestUtilities {
    /// Create mock component
    fn create_mock_component(&self, component_type: &str) -> Box<dyn Component>;

    /// Create test configuration
    fn create_test_config(&self, module_type: &str) -> Config;

    /// Run integration test
    fn run_integration_test(&self, test_case: &TestCase) -> Result<TestResult, Box<dyn std::error::Error>>;

    /// Mock protocol
    fn mock_protocol(&self, protocol_type: &str) -> Box<dyn Protocol>;

    /// Get endpoint information for testing
    fn get_endpoint_info(&self, protocol_id: &str) -> Result<Vec<EndpointInfo>, Box<dyn std::error::Error>>;
}
````

## Logging Contract

### 22. Logging Contract
```rust
pub trait Logger {
    /// Log debug message
    fn debug(&self, message: &str, context: &std::collections::HashMap<String, String>);

    /// Log info message
    fn info(&self, message: &str, context: &std::collections::HashMap<String, String>);

    /// Log warning message
    fn warn(&self, message: &str, context: &std::collections::HashMap<String, String>);

    /// Log error message
    fn error(&self, message: &str, context: &std::collections::HashMap<String, String>);

    /// Log trace message
    fn trace(&self, message: &str, context: &std::collections::HashMap<String, String>);
}
````

## Endpoint Exposure Contract

### 23. Endpoint Discovery Contract
```rust
pub trait EndpointDiscovery {
    /// Get all available endpoints
    fn all_endpoints(&self) -> Result<Vec<EndpointInfo>, Box<dyn std::error::Error>>;

    /// Get endpoints for specific protocol
    fn endpoints_by_protocol(&self, protocol: &str) -> Result<Vec<EndpointInfo>, Box<dyn std::error::Error>>;

    /// Get endpoint by identifier
    fn endpoint_by_id(&self, id: &str) -> Result<EndpointInfo, Box<dyn std::error::Error>>;

    /// Get endpoints by environment
    fn endpoints_by_environment(&self, environment: &str) -> Result<Vec<EndpointInfo>, Box<dyn std::error::Error>>;

    /// Get endpoints by status
    fn endpoints_by_status(&self, status: &EndpointStatus) -> Result<Vec<EndpointInfo>, Box<dyn std::error::Error>>;
}
````

## Contract Compliance

All contracts are designed to:
- Support Spring Boot style dependency injection patterns
- Enable modular architecture with clear interfaces
- Allow customization and override of core modules
- Support multi-module usage without conflicts
- Provide testing utilities and frameworks for module development
- Include configuration management that supports multiple environments
- Provide logging and monitoring capabilities across all modules
- Support versioning and release management for individual modules
- **Support endpoint exposure for client-side discovery** - This is the new key addition that enables clients to discover available server endpoints