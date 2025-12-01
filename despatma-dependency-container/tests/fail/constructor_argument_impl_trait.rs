trait DAL {}
struct PostgresDAL;
impl DAL for PostgresDAL {}
struct Service<D: DAL> {
    dal: D,
}

impl<D: DAL> Service<D> {
    fn new(port: u32, dal: D) -> Self {
        println!("Box dyn Trait service started on port {}", port);
        Self { dal }
    }
}
#[despatma_dependency_container::dependency_container]
impl DependencyContainer {
    fn new(dal: impl DAL) {}

    fn service(&self, dal: impl DAL) -> Service<impl DAL> {
        Service::new(config.port, dal)
    }
}

fn main() {
    let dal: Box<dyn DAL> = Box::new(PostgresDAL);
    let container = DependencyContainer::new(dal);
    let _service = container.service();
}
