struct Config {
    port: u32,
}

struct Service;

impl Service {
    fn new(port: u32) -> Self {
        println!("Service started on port {} with side effect", port);
        Self
    }
}

#[despatma_dependency_container::dependency_container]
impl DependencyContainer {
    fn config(&self) -> Config {
        Config { port: 8080 }
    }

    #[Singleton]
    fn _tracing(&self) -> () {
        println!("Tracing enabled");
    }

    fn service(&self, _tracing: &(), config: Config) -> Service {
        Service::new(config.port)
    }
}

fn main() {
    let container = DependencyContainer::new();
    let _service = container.service();
}
