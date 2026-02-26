# Reusable Library

A Spring Boot-style reusable library framework for Rust applications.

## Overview

This library provides a modular architecture with dependency injection,
configuration management, and support for multiple protocols (API, gRPC).

## Features

- **Dependency Injection**: Container-based dependency injection with registration and resolution
- **Configuration Management**: Flexible configuration system with multiple sources
- **Modular Design**: Clean separation of concerns with modules for API, gRPC, and core functionality
- **Error Handling**: Standardized error types and results
- **Logging**: Structured logging with tracing support

## Usage

```rust
use reusable_library::di::container::Container;
use reusable_library::config::Config;

// Create a container
let mut container = Container::new();

// Register a service
container.register("my_service", MyService::new());

// Resolve a service
let service = container.resolve::<MyService>("my_service");
```

## Modules

- `modules`: Core modules for API, gRPC, and core functionality
- `di`: Dependency injection with container, registry, and injector
- `config`: Configuration management
- `utils`: Utility modules for logging and error handling

## License

MIT