use auto_impl::auto_impl;

#[auto_impl(&)]
trait DAL {}

struct PostgresDAL;

impl DAL for PostgresDAL {}

struct Service<D: DAL> {
    dal: D,
}

impl<D: DAL> Service<D> {
    fn new(dal: D) -> Self {
        println!("Lifetime service with managed impl trait generic started",);
        Self { dal }
    }
}

#[despatma_dependency_container::dependency_container]
impl DependencyContainer {
    #[Singleton(PostgresDAL)]
    fn dal(&self) -> impl DAL {
        PostgresDAL
    }

    #[Scoped]
    fn service(&self, dal: impl DAL) -> Service<impl DAL> {
        Service::new(dal)
    }
}

fn main() {
    let container = DependencyContainer::new();
    let _service = container.service();
}
