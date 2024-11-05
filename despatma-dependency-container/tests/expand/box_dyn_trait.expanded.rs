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
struct DependencyContainer<'a> {
    _phantom: std::marker::PhantomData<&'a ()>,
}
#[automatically_derived]
impl<'a> ::core::clone::Clone for DependencyContainer<'a> {
    #[inline]
    fn clone(&self) -> DependencyContainer<'a> {
        DependencyContainer {
            _phantom: ::core::clone::Clone::clone(&self._phantom),
        }
    }
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
    pub fn config(&'a self) -> Config {
        Config { port: 8080 }
    }
    pub fn dal(&'a self) -> std::boxed::Box<dyn DAL> {
        let b: Box<dyn DAL> = if true {
            Box::new(PostgresDAL)
        } else {
            Box::new(SQLiteDAL)
        };
        b
    }
    pub fn service(&'a self) -> Service<impl DAL> {
        let config = Config { port: 8080 };
        let dal = {
            let b: Box<dyn DAL> = if true {
                Box::new(PostgresDAL)
            } else {
                Box::new(SQLiteDAL)
            };
            b
        };
        Service::new(config.port, dal)
    }
}
fn main() {
    let container = DependencyContainer::new();
    let _service = container.service();
}
