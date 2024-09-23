struct Config {
    port: u32,
}
struct Service;
impl Service {
    fn new(port: u32) -> Self {
        {
            ::std::io::_print(
                format_args!("Service (singleton config) started on port {0}\n", port),
            );
        };
        Self
    }
}
struct DependencyContainer<'a> {
    config: std::rc::Rc<std::cell::OnceCell<Config>>,
    _phantom: std::marker::PhantomData<&'a ()>,
}
impl<'a> DependencyContainer<'a> {
    pub fn new() -> Self {
        Self {
            config: Default::default(),
            _phantom: Default::default(),
        }
    }
    pub fn new_scope(&self) -> Self {
        Self {
            config: self.config.clone(),
            _phantom: Default::default(),
        }
    }
    fn create_config(&self) -> Config {
        Config { port: 8080 }
    }
    pub fn config(&self) -> &Config {
        self.config.get_or_init(|| self.create_config())
    }
    fn create_service(&self, config: &Config) -> Service {
        Service::new(config.port)
    }
    pub fn service(&self) -> Service {
        let config = self.config();
        self.create_service(config)
    }
}
fn main() {
    let container = DependencyContainer::new();
    let _service = container.service();
}
