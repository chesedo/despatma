use auto_impl::auto_impl;

struct Config {
    port: u32,
}

#[auto_impl(Box)]
trait DAL {}

struct PostgresDAL;

impl DAL for PostgresDAL {}

struct SQLiteDAL;

impl DAL for SQLiteDAL {}

struct Service<D: DAL> {
    dal: D,
}

impl<D: DAL> Service<D> {
    fn new(port: u32, dal: D) -> Self {
        println!("Box dyn Trait rpit service started on port {}", port);
        Self { dal }
    }
}

#[despatma_dependency_container::dependency_container]
impl DependencyContainer {
    fn config(&self) -> Config {
        Config { port: 8080 }
    }

    fn dal(&self) -> impl DAL {
        let d: Box<dyn DAL> = if true {
            Box::new(PostgresDAL)
        } else {
            Box::new(SQLiteDAL)
        };

        d
    }

    fn service(&self, config: Config, dal: impl DAL) -> Service<impl DAL> {
        Service::new(config.port, dal)
    }
}

fn main() {
    let container = DependencyContainer::new();
    let _service = container.service();
}
