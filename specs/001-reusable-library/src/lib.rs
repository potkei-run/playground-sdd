//! Main library module for the reusable library with Spring Boot style framework
//!
//! This library provides a modular architecture with dependency injection,
//! configuration management, and support for multiple protocols (API, gRPC).

pub mod modules;
pub mod di;
pub mod config;
pub mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}