struct Config {
    port: u32,
    database_url: String,
}
struct Repository {
    database_url: String,
}
impl Repository {
    fn new(database_url: String) -> Self {
        Self { database_url }
    }
}
struct Service;
impl Service {
    fn new(port: u32) -> Self {
        {
            ::std::io::_print(format_args!("Service started on port {0}\n", port));
        };
        Self
    }
}
struct DependencyContainer<'a> {
    repository: std::rc::Rc<std::cell::OnceCell<Repository>>,
    config: std::sync::Arc<Config>,
    _phantom: std::marker::PhantomData<&'a ()>,
}
#[automatically_derived]
impl<'a> ::core::clone::Clone for DependencyContainer<'a> {
    #[inline]
    fn clone(&self) -> DependencyContainer<'a> {
        DependencyContainer {
            repository: ::core::clone::Clone::clone(&self.repository),
            config: ::core::clone::Clone::clone(&self.config),
            _phantom: ::core::clone::Clone::clone(&self._phantom),
        }
    }
}
impl<'a> DependencyContainer<'a> {
    pub fn new(config: Config) -> Self {
        Self {
            repository: Default::default(),
            config: std::sync::Arc::new(config),
            _phantom: Default::default(),
        }
    }
    pub fn new_scope(&self) -> Self {
        Self {
            repository: self.repository.clone(),
            config: self.config.clone(),
            _phantom: Default::default(),
        }
    }
    pub fn repository(&'a self) -> &Repository {
        let config = self.config.as_ref();
        self.repository.get_or_init(|| { Repository::new(config.database_url.clone()) })
    }
    pub fn service(&'a self) -> Service {
        let config = self.config.as_ref();
        let repository = {
            let config = self.config.as_ref();
            self.repository
                .get_or_init(|| { Repository::new(config.database_url.clone()) })
        };
        Service::new(config.port)
    }
    pub fn config(&'a self) -> &Config {
        self.config.as_ref()
    }
}
fn main() {
    let config = Config {
        port: 8080,
        database_url: "some string".into(),
    };
    let container = DependencyContainer::new(config);
    let _service = container.service();
}
