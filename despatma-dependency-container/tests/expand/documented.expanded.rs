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
struct DependencyContainer<'a> {
    _phantom: std::marker::PhantomData<&'a ()>,
}
impl<'a> DependencyContainer<'a> {
    pub fn new() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
    pub fn new_scope(&self) -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
    /// Creates a new instance of the service.
    pub fn service(&self) -> Service {
        Service::new()
    }
}
fn main() {
    let container = DependencyContainer::new();
    let _service = container.service();
}
