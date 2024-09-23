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
        {
            ::std::io::_print(
                format_args!("Impl trait service started on port {0}\n", port),
            );
        };
        Self { dal }
    }
}
struct DependencyContainer<'a> {
    _phantom: std::marker::PhantomData<&'a ()>,
}
impl<'a> DependencyContainer<'a> {
    pub fn new() -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
    pub fn new_scope(&self) -> Self {
        Self {
            _phantom: Default::default(),
        }
    }
    pub fn config(&self) -> Config {
        { Config { port: 8080 } }
    }
    pub fn dal(&self) -> impl DAL {
        { PostgresDAL }
    }
    pub fn service(&self) -> Service<impl DAL> {
        let config = self.config();
        let dal = self.dal();
        { Service::new(config.port, dal) }
    }
}
fn main() {
    let container = DependencyContainer::new();
    let _service = container.service();
}
