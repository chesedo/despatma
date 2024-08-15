struct Service;

impl Service {
    fn new() -> Self {
        println!("Documented service started");
        Self
    }
}

/// A dependency container for the application.
#[despatma_dependency_container::dependency_container]
impl DependencyContainer {
    /// Creates a new instance of the service.
    fn service(&self) -> Service {
        Service::new()
    }
}

fn main() {
    let container = DependencyContainer::new();
    let _service = container.service();
}
