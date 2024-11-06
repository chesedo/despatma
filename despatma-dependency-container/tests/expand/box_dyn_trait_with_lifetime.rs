use auto_impl::auto_impl;

#[auto_impl(Box, &)]
trait DAL {}

struct PostgresDAL;

impl DAL for PostgresDAL {}

struct SQLiteDAL;

impl DAL for SQLiteDAL {}

struct Service<D: DAL> {
    dal: D,
}

impl<D: DAL> Service<D> {
    fn new(dal: D) -> Self {
        println!("Box dyn Trait singleton service started");
        Self { dal }
    }
}

#[despatma_dependency_container::dependency_container]
impl DependencyContainer {
    #[Singleton]
    fn dal(&self) -> Box<dyn DAL> {
        if true {
            Box::new(PostgresDAL)
        } else {
            Box::new(SQLiteDAL)
        }
    }

    fn service(&self, dal: &Box<dyn DAL>) -> Service<&Box<dyn DAL>> {
        Service::new(dal)
    }
}

fn main() {
    let container = DependencyContainer::new();
    let _service = container.service();
}
