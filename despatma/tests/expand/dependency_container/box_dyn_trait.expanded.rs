use auto_impl::auto_impl;
struct Config {
    port: u32,
}
trait DAL {}
const _: () = {
    extern crate alloc;
    impl<T: DAL + ?::core::marker::Sized> DAL for alloc::boxed::Box<T> {}
};
struct PostgresDAL;
impl DAL for PostgresDAL {}
struct SQLiteDAL;
impl DAL for SQLiteDAL {}
struct Service<D: DAL> {
    dal: D,
}
impl<D: DAL> Service<D> {
    fn new(port: u32, dal: D) -> Self {
        {
            ::std::io::_print(
                format_args!("Box dyn Trait service started on port {0}\n", port),
            );
        };
        Self { dal }
    }
}
struct DependencyContainer;
impl DependencyContainer {
    fn new() -> Self {
        Self
    }
    fn create_config(&self) -> Config {
        Config { port: 8080 }
    }
    pub fn config(&self) -> Config {
        self.create_config()
    }
    fn create_dal(&self) -> Box<dyn DAL> {
        if true { Box::new(PostgresDAL) } else { Box::new(SQLiteDAL) }
    }
    pub fn dal(&self) -> Box<dyn DAL> {
        self.create_dal()
    }
    fn create_service(&self, config: Config, dal: impl DAL) -> Service<impl DAL> {
        Service::new(config.port, dal)
    }
    pub fn service(&self) -> Service<impl DAL> {
        let config = self.config();
        let dal = self.dal();
        self.create_service(config, dal)
    }
}
fn main() {
    let container = DependencyContainer::new();
    let _service = container.service();
}
