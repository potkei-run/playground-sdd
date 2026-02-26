# Quickstart: Project Framework Update

## Overview

This document provides a quickstart guide for implementing the project framework update that aligns with the established naming convention principles. The update ensures all subproject directories follow the "rootproject-001-subproject" pattern.

## Prerequisites

- Rust 1.75 or later
- Cargo package manager
- Git version control
- Basic understanding of monorepo structure

## Getting Started

### 1. Understanding the Naming Convention

The project follows the naming convention: `rootproject-001-subproject`

For this repository:
- Root project name: `reusable-library`
- Subproject pattern: `reusable-library-001-<subproject-type>`

### 2. Project Structure

The updated project structure will be organized as:

```
reusable-library-001-project-framework-core/
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── config/
│   ├── di/
│   ├── modules/
│   └── utils/
├── tests/
├── Cargo.toml
└── README.md

reusable-library-001-project-framework-api/
├── src/
│   └── api.rs
├── Cargo.toml
└── README.md

reusable-library-001-project-framework-grpc/
├── src/
│   └── grpc.rs
├── Cargo.toml
└── README.md
```

### 3. Implementation Steps

1. **Update Directory Names**:
   - Rename existing project directories to follow the new naming convention
   - Update all internal references to match new directory names

2. **Maintain Backward Compatibility**:
   - Ensure all existing functionality continues to work
   - Update documentation and references

3. **Validate Naming Convention**:
   - Verify all subprojects follow the consistent naming pattern
   - Run validation tests to confirm compliance

### 4. Testing

Run the following commands to verify the implementation:

```bash
# Run all tests
cargo test

# Check code formatting
cargo fmt --check

# Validate dependencies
cargo check
```

## Configuration

### Cargo.toml Structure

Each subproject will have its own Cargo.toml with appropriate dependencies:

```toml
[package]
name = "reusable-library-001-project-framework-core"
version = "0.1.0"
edition = "2021"

[dependencies]
# Add relevant dependencies here
```

## Best Practices

1. **Consistency**: All new subprojects must follow the naming convention
2. **Independence**: Each subproject should be independently buildable and testable
3. **Documentation**: Update documentation to reflect new naming conventions
4. **Testing**: Maintain 100% test coverage for all components

## Troubleshooting

### Common Issues

1. **Directory naming conflicts**:
   - Ensure no existing directories conflict with the new naming convention
   - Resolve any naming conflicts before proceeding

2. **Reference issues**:
   - Update all internal references to subprojects
   - Verify imports and module paths are correct

3. **Versioning problems**:
   - Ensure semantic versioning is maintained across subprojects
   - Update version numbers appropriately

## Next Steps

1. Implement the directory renaming according to the naming convention
2. Update all internal references and documentation
3. Run comprehensive tests to ensure backward compatibility
4. Validate that all subprojects follow the naming convention