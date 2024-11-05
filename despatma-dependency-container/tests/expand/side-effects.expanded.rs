struct Config {
    port: u32,
}
struct Service;
impl Service {
    fn new(port: u32) -> Self {
        {
            ::std::io::_print(
                format_args!("Service started on port {0} with side effect\n", port),
            );
        };
        Self
    }
}
struct DependencyContainer<'a> {
    _tracing: std::rc::Rc<std::cell::OnceCell<()>>,
    _phantom: std::marker::PhantomData<&'a ()>,
}
#[automatically_derived]
impl<'a> ::core::clone::Clone for DependencyContainer<'a> {
    #[inline]
    fn clone(&self) -> DependencyContainer<'a> {
        DependencyContainer {
            _tracing: ::core::clone::Clone::clone(&self._tracing),
            _phantom: ::core::clone::Clone::clone(&self._phantom),
        }
    }
}
impl<'a> DependencyContainer<'a> {
    pub fn new() -> Self {
        Self {
            _tracing: Default::default(),
            _phantom: Default::default(),
        }
    }
    pub fn new_scope(&self) -> Self {
        Self {
            _tracing: self._tracing.clone(),
            _phantom: Default::default(),
        }
    }
    pub fn config(&'a self) -> Config {
        Config { port: 8080 }
    }
    pub fn _tracing(&'a self) -> &() {
        self._tracing
            .get_or_init(|| {
                {
                    ::std::io::_print(format_args!("Tracing enabled\n"));
                };
            })
    }
    pub fn service(&'a self) -> Service {
        let _tracing = self
            ._tracing
            .get_or_init(|| {
                {
                    ::std::io::_print(format_args!("Tracing enabled\n"));
                };
            });
        let config = Config { port: 8080 };
        Service::new(config.port)
    }
}
fn main() {
    let container = DependencyContainer::new();
    let _service = container.service();
}
