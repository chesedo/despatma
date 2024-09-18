use auto_impl::auto_impl;

#[auto_impl(&)]
trait Config {
    fn port(&self) -> u16;
}

struct EnvConfig;

impl Config for EnvConfig {
    fn port(&self) -> u16 {
        8080
    }
}

#[auto_impl(&)]
trait DAL {}

struct PostgresDAL;

impl DAL for PostgresDAL {}

struct Service<C, D> {
    config: C,
    dal: D,
}

impl<C: Config, D: DAL> Service<C, D> {
    fn new(config: C, dal: D) -> Self {
        println!("Service started on port {}", config.port());
        Self { config, dal }
    }
}

#[despatma_dependency_container::dependency_container]
impl DependencyContainer {
    #[Scoped]
    fn config(&self) -> impl Config {
        EnvConfig
    }

    #[Singleton]
    fn dal(&self) -> impl DAL {
        PostgresDAL
    }

    fn service(&self, config: impl Config, dal: impl DAL) -> Service<impl Config, impl DAL> {
        Service::new(config, dal)
    }
}

fn main() {
    let container = DependencyContainer::new();
    let _service = container.service();
}
