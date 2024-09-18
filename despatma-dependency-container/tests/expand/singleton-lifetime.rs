struct Config {
    port: u32,
}

struct Service;

impl Service {
    fn new(port: u32) -> Self {
        println!("Service (singleton config) started on port {}", port);
        Self
    }
}

#[despatma_dependency_container::dependency_container]
impl DependencyContainer {
    #[Singleton]
    fn config(&self) -> Config {
        Config { port: 8080 }
    }

    fn service(&self, config: &Config) -> Service {
        Service::new(config.port)
    }
}

fn main() {
    let container = DependencyContainer::new();
    let _service = container.service();
}
