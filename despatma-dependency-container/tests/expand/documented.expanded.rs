struct Service;
impl Service {
    fn new() -> Self {
        {
            ::std::io::_print(format_args!("Documented service started\n"));
        };
        Self
    }
}
/// A dependency container for the application.
struct DependencyContainer;
impl DependencyContainer {
    pub fn new() -> Self {
        Self
    }
    pub fn new_scope(&self) -> Self {
        Self
    }
    fn create_service(&self) -> Service {
        Service::new()
    }
    /// Creates a new instance of the service.
    pub fn service(&self) -> Service {
        self.create_service()
    }
}
fn main() {
    let container = DependencyContainer::new();
    let _service = container.service();
}
