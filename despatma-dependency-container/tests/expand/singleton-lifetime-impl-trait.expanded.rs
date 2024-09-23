use auto_impl::auto_impl;
struct Config {
    port: u32,
}
trait DAL {}
const _: () = {
    impl<'a, T: 'a + DAL + ?::core::marker::Sized> DAL for &'a T {}
};
struct PostgresDAL;
impl DAL for PostgresDAL {}
struct Service<D> {
    dal: D,
}
impl<D: DAL> Service<D> {
    fn new(port: u32, dal: D) -> Self {
        {
            ::std::io::_print(
                format_args!(
                    "Impl Trait singleton lifetime service started on port {0}\n", port,
                ),
            );
        };
        Self { dal }
    }
}
struct DependencyContainer<'a> {
    dal: std::rc::Rc<std::cell::OnceCell<PostgresDAL>>,
    _phantom: std::marker::PhantomData<&'a ()>,
}
impl<'a> DependencyContainer<'a> {
    pub fn new() -> Self {
        Self {
            dal: Default::default(),
            _phantom: Default::default(),
        }
    }
    pub fn new_scope(&self) -> Self {
        Self {
            dal: self.dal.clone(),
            _phantom: Default::default(),
        }
    }
    fn create_config(&self) -> Config {
        Config { port: 8080 }
    }
    pub fn config(&self) -> Config {
        self.create_config()
    }
    fn create_dal(&self) -> PostgresDAL {
        PostgresDAL
    }
    pub fn dal(&self) -> &impl DAL {
        self.dal.get_or_init(|| self.create_dal())
    }
    fn create_service(&self, config: Config, dal: impl DAL) -> Service<impl DAL> {
        Service::new(config.port, dal)
    }
    pub fn service(&self) -> Service<impl DAL + '_> {
        let config = self.config();
        let dal = self.dal();
        self.create_service(config, dal)
    }
}
fn main() {
    let container = DependencyContainer::new();
    let _service = container.service();
}
