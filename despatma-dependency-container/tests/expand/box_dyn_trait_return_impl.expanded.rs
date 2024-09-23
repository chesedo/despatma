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
                format_args!("Box dyn Trait rpit service started on port {0}\n", port),
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
    fn create_config(&self) -> Config {
        Config { port: 8080 }
    }
    pub fn config(&self) -> Config {
        self.create_config()
    }
    fn create_dal(&self) -> impl DAL {
        let d: Box<dyn DAL> = if true {
            Box::new(PostgresDAL)
        } else {
            Box::new(SQLiteDAL)
        };
        d
    }
    pub fn dal(&self) -> impl DAL {
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
