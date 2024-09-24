use std::time::Duration;
use tokio::time::sleep;
struct Config {
    port: u32,
}
struct Service;
impl Service {
    fn new(port: u32) -> Self {
        {
            ::std::io::_print(
                format_args!("Async singleton service started on port {0}\n", port),
            );
        };
        Self
    }
}
struct DependencyContainer<'a> {
    config: std::sync::Arc<async_once_cell::OnceCell<Config>>,
    _phantom: std::marker::PhantomData<&'a ()>,
}
impl<'a> DependencyContainer<'a> {
    pub fn new() -> Self {
        Self {
            config: Default::default(),
            _phantom: Default::default(),
        }
    }
    pub fn new_scope(&self) -> Self {
        Self {
            config: self.config.clone(),
            _phantom: Default::default(),
        }
    }
    pub async fn config(&self) -> &Config {
        self.config
            .get_or_init(async {
                sleep(Duration::from_millis(10)).await;
                Config { port: 8080 }
            })
            .await
    }
    pub async fn service(&self) -> Service {
        let config = self
            .config
            .get_or_init(async {
                sleep(Duration::from_millis(10)).await;
                Config { port: 8080 }
            })
            .await;
        Service::new(config.port)
    }
}
fn main() {
    let body = async {
        let container = DependencyContainer::new();
        let _service = container.service().await;
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
