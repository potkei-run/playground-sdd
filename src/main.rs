//! Main entry point for the reusable library framework.

use reusable_library::prelude::*;

fn main() {
    println!("Reusable Library Framework v{}", reusable_library::VERSION);

    // Example usage of the framework
    let config = ContainerConfig {
        auto_scan: true,
        scan_paths: vec!["src".to_string()],
        default_scope: Scope::Singleton,
        enable_caching: true,
    };

    let container = DiContainerImpl::new(config);

    // Create a simple module
    let module = LibraryModule::new(
        "example-module".to_string(),
        "Example Module".to_string(),
        "1.0.0".to_string(),
        "An example module".to_string(),
        vec![],
        ModuleType::Core,
    );

    println!("Created module: {}", module.name());
    println!("Module status: {:?}", module.status());
}