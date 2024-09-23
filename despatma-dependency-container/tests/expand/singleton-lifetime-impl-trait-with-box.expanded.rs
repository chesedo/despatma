use auto_impl::auto_impl;
struct Config {
    port: u32,
}
trait DAL {}
const _: () = {
    extern crate alloc;
    impl<T: DAL + ?::core::marker::Sized> DAL for alloc::boxed::Box<T> {}
};
const _: () = {
    impl<'a, T: 'a + DAL + ?::core::marker::Sized> DAL for &'a T {}
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
                format_args!(
                    "Box dyn Trait singleton lifetime service started on port {0}\n",
                    port,
                ),
            );
        };
        Self { dal }
    }
}
struct DependencyContainer<'a> {
    dal: std::rc::Rc<std::cell::OnceCell<Box<dyn DAL>>>,
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
    pub fn config(&self) -> Config {
        { Config { port: 8080 } }
    }
    pub fn dal(&self) -> &impl DAL {
        self.dal
            .get_or_init(|| {
                if true { Box::new(PostgresDAL) } else { Box::new(SQLiteDAL) }
            })
    }
    pub fn service(&self) -> Service<impl DAL + '_> {
        let config = self.config();
        let dal = self.dal();
        { Service::new(config.port, dal) }
    }
}
fn main() {
    let container = DependencyContainer::new();
    let _service = container.service();
}
