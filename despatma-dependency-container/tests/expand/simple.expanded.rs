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
struct DependencyContainer;
impl DependencyContainer {
    fn new() -> Self {
        Self
    }
    pub fn new_scope(&self) -> Self {
        Self
    }
    fn create_config(&self) -> Config {
        Config { port: 8080 }
    }
    pub fn config(&self) -> Config {
        self.create_config()
    }
    fn create_service(&self, config: Config) -> Service {
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
