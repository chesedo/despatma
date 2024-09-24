struct Config {
    port: u32,
}
struct Service;
impl Service {
    fn new(port: u32) -> Self {
        {
            ::std::io::_print(format_args!("Service started on port {0}\n", port));
        };
        Self
    }
}
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
    pub fn config(&self) -> Config {
        Config { port: 8080 }
    }
    pub fn service(&self) -> Service {
        let config = Config { port: 8080 };
        Service::new(config.port)
    }
}
fn main() {
    let container = DependencyContainer::new();
    let _service = container.service();
}
