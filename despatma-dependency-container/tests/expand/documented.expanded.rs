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
#[automatically_derived]
impl<'a> ::core::clone::Clone for DependencyContainer<'a> {
    #[inline]
    fn clone(&self) -> DependencyContainer<'a> {
        DependencyContainer {
            _phantom: ::core::clone::Clone::clone(&self._phantom),
        }
    }
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
    pub fn service(&'a self) -> Service {
        Service::new()
    }
}
fn main() {
    let container = DependencyContainer::new();
    let _service = container.service();
}
