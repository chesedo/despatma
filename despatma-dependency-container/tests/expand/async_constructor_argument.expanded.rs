use std::time::Duration;
use tokio::time::sleep;
struct Config {
    port: u32,
}
struct Service;
impl Service {
    fn new(port: u32) -> Self {
        {
            ::std::io::_print(format_args!("Async service started on port {0}\n", port));
        };
        Self
    }
}
struct DependencyContainer<'a> {
    config: std::sync::Arc<Config>,
    _phantom: std::marker::PhantomData<&'a ()>,
}
#[automatically_derived]
impl<'a> ::core::clone::Clone for DependencyContainer<'a> {
    #[inline]
    fn clone(&self) -> DependencyContainer<'a> {
        DependencyContainer {
            config: ::core::clone::Clone::clone(&self.config),
            _phantom: ::core::clone::Clone::clone(&self._phantom),
        }
    }
}
impl<'a> DependencyContainer<'a> {
    pub fn new(config: Config) -> Self {
        Self {
            config: std::sync::Arc::new(config),
            _phantom: Default::default(),
        }
    }
    pub fn new_scope(&self) -> Self {
        Self {
            config: self.config.clone(),
            _phantom: Default::default(),
        }
    }
    pub async fn service(&'a self) -> Service {
        let config = self.config.as_ref();
        sleep(Duration::from_millis(10)).await;
        Service::new(config.port)
    }
    pub fn config(&'a self) -> &Config {
        self.config.as_ref()
    }
}
fn main() {
    let body = async {
        let config = Config { port: 8080 };
        let container = DependencyContainer::new(config);
        let _service = container.service().await;
    };
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return,
        clippy::unwrap_in_result
    )]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
