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
                format_args!(
                    "Impl trait but registering concrete service started on port {0}\n",
                    port,
                ),
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
    pub fn dal(&'a self) -> PostgresDAL {
        PostgresDAL
    }
    pub fn service(&'a self) -> Service<impl DAL> {
        let config = Config { port: 8080 };
        let dal = PostgresDAL;
        Service::new(config.port, dal)
    }
}
fn main() {
    let container = DependencyContainer::new();
    let _service = container.service();
}
