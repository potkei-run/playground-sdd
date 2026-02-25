# Data Model: Reusable Library with Spring Boot Style Framework

## Overview

This document defines the core data models for the reusable library framework that supports modular architecture with Spring Boot style dependency injection and multiple protocol support (API, gRPC, MCP).

## Core Entities

### 1. Library Module
A self-contained component that provides specific functionality with clear interfaces and dependencies.

```rust
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
}
```

### 2. Module Configuration
Configuration settings specific to each module.

```rust
pub struct ModuleConfig {
    /// Module-specific settings
    pub settings: std::collections::HashMap<String, serde_json::Value>,

    /// Environment-specific overrides
    pub environments: std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>,

    /// Feature flags for module capabilities
    pub features: FeatureFlags,
}
```

### 3. Dependency Injection Container
System that manages module instantiation and dependency resolution.

```rust
pub struct DiContainer {
    /// Registered components
    pub components: std::collections::HashMap<String, Component>,

    /// Component instances (singleton scope)
    pub instances: std::collections::HashMap<String, Box<dyn Any>>,

    /// Configuration for the container
    pub config: ContainerConfig,
}
```

### 4. Component
A registered component that can be injected.

```rust
pub struct Component {
    /// Component identifier
    pub id: String,

    /// Component type (trait or concrete type)
    pub component_type: ComponentType,

    /// Scope of the component (transient, singleton, scoped)
    pub scope: Scope,

    /// Dependencies required by this component
    pub dependencies: Vec<String>,

    /// Factory function to create the component
    pub factory: Box<dyn Fn(&DiContainer) -> Box<dyn Any>>,

    /// Component metadata
    pub metadata: std::collections::HashMap<String, String>,
}
```

### 5. Protocol
Communication interface abstraction.

```rust
pub struct Protocol {
    /// Protocol identifier
    pub id: String,

    /// Protocol name (API, gRPC, MCP)
    pub name: String,

    /// Protocol configuration
    pub config: ProtocolConfig,

    /// Protocol status
    pub status: ProtocolStatus,

    /// Associated services
    pub services: Vec<String>,

    /// Endpoint information for client discovery
    pub endpoints: Vec<EndpointInfo>,
}
```

### 6. Protocol Configuration
Configuration for protocol-specific settings.

```rust
pub struct ProtocolConfig {
    /// Protocol-specific settings
    pub settings: std::collections::HashMap<String, serde_json::Value>,

    /// Network configuration
    pub network: NetworkConfig,

    /// Security configuration
    pub security: SecurityConfig,

    /// Performance settings
    pub performance: PerformanceConfig,
}
```

### 7. Service
Business logic component that provides functionality.

```rust
pub struct Service {
    /// Service identifier
    pub id: String,

    /// Service name
    pub name: String,

    /// Service description
    pub description: String,

    /// Service dependencies
    pub dependencies: Vec<String>,

    /// Service configuration
    pub config: ServiceConfig,

    /// Service status
    pub status: ServiceStatus,
}
```

### 8. Configuration Manager
System that handles environment-specific settings and module configurations.

```rust
pub struct ConfigManager {
    /// Base configuration
    pub base_config: std::collections::HashMap<String, serde_json::Value>,

    /// Environment-specific configurations
    pub environments: std::collections::HashMap<String, std::collections::HashMap<String, serde_json::Value>>,

    /// Configuration sources
    pub sources: Vec<ConfigSource>,

    /// Configuration validation rules
    pub validation: ValidationRules,
}
```

### 9. Endpoint Information
Information about available endpoints for client-side discovery.

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
```

### 10. Endpoint Parameter
Definition of endpoint parameters.

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
```

### 11. Endpoint Type
Enumeration of endpoint types.

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
```

### 12. Endpoint Status
Status of endpoints.

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
```

### 13. Security Requirement
Security requirements for endpoints.

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
```

## Enum Definitions

### Module Type
```rust
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
```

### Scope
```rust
pub enum Scope {
    /// Transient - new instance each time
    Transient,

    /// Singleton - single instance for entire application
    Singleton,

    /// Scoped - instance per scope (request, session, etc.)
    Scoped,
}
```

### Protocol Status
```rust
pub enum ProtocolStatus {
    /// Protocol is stopped
    Stopped,

    /// Protocol is starting
    Starting,

    /// Protocol is running
    Running,

    /// Protocol has errored
    Error(String),
}
```

### Service Status
```rust
pub enum ServiceStatus {
    /// Service is disabled
    Disabled,

    /// Service is initializing
    Initializing,

    /// Service is ready
    Ready,

    /// Service is in error state
    Error(String),
}
```

## Relationships

### Module to Component
- One-to-many: A module can contain multiple components
- Components are registered with the DI container

### Component to Service
- Many-to-many: Components can depend on services, services can use components
- Dependency injection resolves these relationships

### Protocol to Service
- Many-to-many: Protocols can expose multiple services
- Services can be exposed through multiple protocols

### Protocol to Endpoint
- One-to-many: A protocol can expose multiple endpoints
- Endpoints are registered with the protocol

### Configuration to Module
- One-to-one: Each module has its own configuration
- Configuration can be overridden per environment

## Validation Rules

### Module Validation
- Module ID must be unique
- Version must follow semantic versioning
- Dependencies must resolve correctly
- Configuration must be valid

### Component Validation
- Component ID must be unique within container
- Dependencies must be registered components
- Factory function must be valid
- Scope must be valid

### Protocol Validation
- Protocol must have valid configuration
- Network settings must be valid
- Security settings must be configured
- Port must be available

### Endpoint Validation
- Endpoint ID must be unique within protocol
- Endpoint path must be valid
- Endpoint parameters must be properly defined
- Security requirements must be valid

## State Transitions

### Module Lifecycle
```
Disabled → Enabled → Ready → Error → Disabled
```

### Component Lifecycle
```
Registered → Initialized → Ready → Error → Unregistered
```

### Protocol Lifecycle
```
Stopped → Starting → Running → Error → Stopped
```

## Data Flow

1. **Configuration Loading**: Configuration manager loads settings from sources
2. **Module Registration**: Modules are registered with DI container
3. **Component Resolution**: Dependencies are resolved through DI container
4. **Protocol Initialization**: Protocols are initialized with their configurations
5. **Endpoint Registration**: Endpoints are registered with protocols
6. **Service Activation**: Services are activated and made available
7. **Protocol Start**: Protocols are started and begin accepting connections

## Security Considerations

- All configuration data should be validated before use
- Sensitive information should be encrypted in configuration files
- Access control should be implemented for service endpoints
- Protocol-specific security settings should be enforced
- Endpoint information should be properly secured

## Performance Considerations

- Component instantiation should be optimized for performance
- Configuration loading should be cached where possible
- Protocol startup should be efficient
- Memory usage should be monitored for all components
- Endpoint discovery should be fast and efficient

## Endpoint Exposure Features

The framework now includes comprehensive endpoint exposure capabilities:

### 1. Endpoint Discovery
- Endpoints are automatically registered with protocols
- Each endpoint includes detailed metadata
- Protocol-level endpoint information collection

### 2. Client-Side Integration
- Standardized endpoint information for client discovery
- Support for multiple endpoint types (HTTP, gRPC, WebSocket)
- Environment-specific endpoint availability

### 3. Metadata Exposure
- Endpoint parameters (input/output)
- Security requirements
- Status information
- Protocol association
- Environment availability

### 4. Dynamic Endpoint Management
- Endpoints can be enabled/disabled
- Endpoint status can be updated
- Endpoint information can be modified at runtime