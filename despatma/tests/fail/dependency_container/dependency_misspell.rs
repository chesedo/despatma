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

#[despatma::dependency_container]
impl DependencyContainer {
    fn config(&self) -> Config {
        Config { port: 8080 }
    }

    fn service(&self, canfi: Config) -> Service {
        Service::new(config.port)
    }
}
