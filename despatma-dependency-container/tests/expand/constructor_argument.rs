struct Config {
    port: u32,
    database_url: String,
}

struct Repository {
    database_url: String,
}

impl Repository {
    fn new(database_url: String) -> Self {
        Self { database_url }
    }
}

struct Service;

impl Service {
    fn new(port: u32) -> Self {
        println!("Service started on port {}", port);
        Self
    }
}

#[despatma_dependency_container::dependency_container]
impl DependencyContainer {
    fn new(config: Config) {}

    #[Singleton]
    fn repository(&self, config: &Config) -> Repository {
        Repository::new(config.database_url.clone())
    }
    fn service(&self, config: &Config, repository: &Repository) -> Service {
        Service::new(config.port)
    }
}

fn main() {
    let config = Config {
        port: 8080,
        database_url: "some string".into(),
    };
    let container = DependencyContainer::new(config);
    let _service = container.service();
}
