use auto_impl::auto_impl;
trait DAL {}
const _: () = {
    impl<'a, T: 'a + DAL + ?::core::marker::Sized> DAL for &'a T {}
};
struct PostgresDAL;
impl DAL for PostgresDAL {}
struct Service<D: DAL> {
    dal: D,
}
impl<D: DAL> Service<D> {
    fn new(dal: D) -> Self {
        {
            ::std::io::_print(
                format_args!(
                    "Lifetime service with managed impl trait generic started\n",
                ),
            );
        };
        Self { dal }
    }
}
struct DependencyContainer<'a> {
    dal: std::rc::Rc<std::cell::OnceCell<PostgresDAL>>,
    service: std::rc::Rc<std::cell::OnceCell<Service<&'a PostgresDAL>>>,
    _phantom: std::marker::PhantomData<&'a ()>,
}
#[automatically_derived]
impl<'a> ::core::clone::Clone for DependencyContainer<'a> {
    #[inline]
    fn clone(&self) -> DependencyContainer<'a> {
        DependencyContainer {
            dal: ::core::clone::Clone::clone(&self.dal),
            service: ::core::clone::Clone::clone(&self.service),
            _phantom: ::core::clone::Clone::clone(&self._phantom),
        }
    }
}
impl<'a> DependencyContainer<'a> {
    pub fn new() -> Self {
        Self {
            dal: Default::default(),
            service: Default::default(),
            _phantom: Default::default(),
        }
    }
    pub fn new_scope(&self) -> Self {
        Self {
            dal: self.dal.clone(),
            service: Default::default(),
            _phantom: Default::default(),
        }
    }
    pub fn dal(&'a self) -> &impl DAL {
        self.dal.get_or_init(|| { PostgresDAL })
    }
    pub fn service(&'a self) -> &Service<impl DAL + use<'a>> {
        let dal = self.dal.get_or_init(|| { PostgresDAL });
        self.service.get_or_init(|| { Service::new(dal) })
    }
}
fn main() {
    let container = DependencyContainer::new();
    let _service = container.service();
}
