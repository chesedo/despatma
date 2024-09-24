struct Config {
    port: u32,
}

trait DAL {}

struct PostgresDAL;

impl DAL for PostgresDAL {}

struct Service<D: DAL> {
    dal: D,
}

impl<D: DAL> Service<D> {
    fn new(port: u32, dal: D) -> Self {
        println!("Impl trait service started on port {}", port);
        Self { dal }
    }
}

#[despatma_dependency_container::dependency_container]
impl DependencyContainer {
    fn config(&self) -> Config {
        Config { port: 8080 }
    }

    #[Transient(PostgresDAL)]
    fn dal(&self) -> impl DAL {
        PostgresDAL
    }

    fn service(&self, config: Config, dal: PostgresDAL) -> Service<PostgresDAL> {
        Service::new(config.port, dal)
    }
}

fn main() {
    let container = DependencyContainer::new();
    let _service = container.service();
}
