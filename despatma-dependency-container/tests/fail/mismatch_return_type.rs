struct Config {
    port: u32,
}

struct Unit;

struct Service;

impl Service {
    fn new(port: u32, _unit: Unit) -> Self {
        println!("Service started on port {}", port);
        Self
    }
}

#[despatma_dependency_container::dependency_container]
impl DependencyContainer {
    fn config(&self) -> Config {
        Config { port: 8080 }
    }

    fn unit(&self) -> () {
        ()
    }

    fn service(&self, config: u32, unit: Unit) -> Service {
        Service::new(config, unit)
    }
}

fn main() {
    let container = DependencyContainer::new();
    let _service = container.service();
}
