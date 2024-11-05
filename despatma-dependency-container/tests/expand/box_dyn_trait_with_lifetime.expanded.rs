use auto_impl::auto_impl;
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
    fn new(dal: D) -> Self {
        {
            ::std::io::_print(format_args!("Box dyn Trait singleton service started\n"));
        };
        Self { dal }
    }
}
struct DependencyContainer<'a> {
    dal: std::rc::Rc<std::cell::OnceCell<std::boxed::Box<dyn DAL>>>,
    _phantom: std::marker::PhantomData<&'a ()>,
}
#[automatically_derived]
impl<'a> ::core::clone::Clone for DependencyContainer<'a> {
    #[inline]
    fn clone(&self) -> DependencyContainer<'a> {
        DependencyContainer {
            dal: ::core::clone::Clone::clone(&self.dal),
            _phantom: ::core::clone::Clone::clone(&self._phantom),
        }
    }
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
    pub fn dal(&'a self) -> &std::boxed::Box<dyn DAL> {
        self.dal
            .get_or_init(|| {
                if true { Box::new(PostgresDAL) } else { Box::new(SQLiteDAL) }
            })
    }
    pub fn service(&'a self) -> Service<&Box<dyn DAL>> {
        let dal = self
            .dal
            .get_or_init(|| {
                if true { Box::new(PostgresDAL) } else { Box::new(SQLiteDAL) }
            });
        Service::new(dal)
    }
}
fn main() {
    let container = DependencyContainer::new();
    let _service = container.service();
}
