use std::time::Duration;
use tokio::time::sleep;

struct Config {
    port: u32,
}

struct Service;

impl Service {
    fn new(port: u32) -> Self {
        println!("Async service started on port {}", port);
        Self
    }
}

#[despatma_dependency_container::dependency_container]
impl DependencyContainer {
    fn new(config: Config) {}

    async fn service(&self, config: &Config) -> Service {
        sleep(Duration::from_millis(10)).await;
        Service::new(config.port)
    }
}

#[tokio::main]
async fn main() {
    let config = Config { port: 8080 };
    let container = DependencyContainer::new(config);
    let _service = container.service().await;
}
