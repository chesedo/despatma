struct Config {
    port: u32,
}

struct Service;

impl Service {
    fn new(port: u32) -> Self {
        println!("Service started on port {}", port);
        Self
    }
}

#[despatma_dependency_container::dependency_container]
impl ConfigContainer {
    fn config(&self) -> Config {
        Config { port: 8080 }
    }
}

#[despatma_dependency_container::dependency_container]
impl ServiceContainer {
    fn new(config_container: ConfigContainer<'static>) {}

    fn service(&self, config_container: &ConfigContainer) -> Service {
        Service::new(config_container.config().port)
    }
}

fn main() {
    let config_container = ConfigContainer::new();
    let container = ServiceContainer::new(config_container);
    let _service = container.service();
}
