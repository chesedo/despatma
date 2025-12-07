struct Config {
    port: u32,
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
struct ConfigContainer<'a> {
    _phantom: std::marker::PhantomData<&'a ()>,
}
#[automatically_derived]
impl<'a> ::core::clone::Clone for ConfigContainer<'a> {
    #[inline]
    fn clone(&self) -> ConfigContainer<'a> {
        ConfigContainer {
            _phantom: ::core::clone::Clone::clone(&self._phantom),
        }
    }
}
impl<'a> ConfigContainer<'a> {
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
}
struct ServiceContainer<'a> {
    config_container: std::sync::Arc<ConfigContainer<'a>>,
    _phantom: std::marker::PhantomData<&'a ()>,
}
#[automatically_derived]
impl<'a> ::core::clone::Clone for ServiceContainer<'a> {
    #[inline]
    fn clone(&self) -> ServiceContainer<'a> {
        ServiceContainer {
            config_container: ::core::clone::Clone::clone(&self.config_container),
            _phantom: ::core::clone::Clone::clone(&self._phantom),
        }
    }
}
impl<'a> ServiceContainer<'a> {
    pub fn new(config_container: ConfigContainer<'a>) -> Self {
        Self {
            config_container: std::sync::Arc::new(config_container),
            _phantom: Default::default(),
        }
    }
    pub fn new_scope(&self) -> Self {
        Self {
            config_container: self.config_container.clone(),
            _phantom: Default::default(),
        }
    }
    pub fn service(&'a self) -> Service {
        let config_container = self.config_container.as_ref();
        Service::new(config_container.config().port)
    }
    pub fn config_container(&'a self) -> &ConfigContainer<'a> {
        self.config_container.as_ref()
    }
}
fn main() {
    let config_container = ConfigContainer::new();
    let container = ServiceContainer::new(config_container);
    let _service = container.service();
}
