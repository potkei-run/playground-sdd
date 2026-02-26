//! Example demonstrating usage of the reusable library
//!
//! This example shows how to use the core features of the reusable library
//! including dependency injection and configuration management.

use reusable_library::di::container::Container;
use reusable_library::config::Config;
use reusable_library::utils::logging;

/// A simple service that can be injected
struct MyService {
    name: String,
}

impl MyService {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    fn do_work(&self) -> String {
        format!("{} is working", self.name)
    }
}

fn main() {
    // Initialize logging
    logging::init_logging();
    logging::log_info("Starting example application");

    // Create a configuration
    let mut config = Config::new();
    config.set("app_name", "Reusable Library Example");
    config.set("version", "1.0.0");

    logging::log_info(&format!("App name: {:?}", config.get("app_name")));

    // Create dependency injection container
    let mut container = Container::new();

    // Register a service
    let service = MyService::new("ExampleService");
    container.register("my_service", service);

    // Resolve the service
    let resolved_service = container.resolve::<MyService>("my_service");
    if let Some(service) = resolved_service {
        logging::log_info(&service.do_work());
    }

    logging::log_info("Example application finished");
}