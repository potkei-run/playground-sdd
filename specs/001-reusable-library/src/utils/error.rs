//! Core error handling and result types
//!
//! This module provides standardized error handling for the reusable library framework.

use std::fmt;

/// Result type for the library
pub type Result<T> = std::result::Result<T, LibraryError>;

/// Library error type
#[derive(Debug, Clone)]
pub enum LibraryError {
    /// Configuration error
    ConfigError(String),
    /// Dependency injection error
    DependencyError(String),
    /// Module error
    ModuleError(String),
    /// IO error
    IoError(String),
    /// Generic error
    GenericError(String),
}

impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LibraryError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            LibraryError::DependencyError(msg) => write!(f, "Dependency error: {}", msg),
            LibraryError::ModuleError(msg) => write!(f, "Module error: {}", msg),
            LibraryError::IoError(msg) => write!(f, "IO error: {}", msg),
            LibraryError::GenericError(msg) => write!(f, "Generic error: {}", msg),
        }
    }
}

impl std::error::Error for LibraryError {}

impl From<std::io::Error> for LibraryError {
    fn from(error: std::io::Error) -> Self {
        LibraryError::IoError(error.to_string())
    }
}

impl From<anyhow::Error> for LibraryError {
    fn from(error: anyhow::Error) -> Self {
        LibraryError::GenericError(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let config_error = LibraryError::ConfigError("test config".to_string());
        let dependency_error = LibraryError::DependencyError("test dependency".to_string());

        assert_eq!(format!("{}", config_error), "Configuration error: test config");
        assert_eq!(format!("{}", dependency_error), "Dependency error: test dependency");
    }
}