//! This test creates a multiline function arg to ensure trailing commas are handled correctly
struct Configuration {
    port: u32,
}
struct MyDataLayerOverSocket;
struct Service;
impl Service {
    fn new(port: u32, _my_data_layer_over_socket: MyDataLayerOverSocket) -> Self {
        {
            ::std::io::_print(
                format_args!("Trailing comma service started on port {0}\n", port),
            );
        };
        Self
    }
}
struct DependencyContainer<'a> {
    _phantom: std::marker::PhantomData<&'a ()>,
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
    fn create_configuration(&self) -> Configuration {
        Configuration { port: 8080 }
    }
    pub fn configuration(&self) -> Configuration {
        self.create_configuration()
    }
    fn create_my_data_layer_over_socket(&self) -> MyDataLayerOverSocket {
        MyDataLayerOverSocket
    }
    pub fn my_data_layer_over_socket(&self) -> MyDataLayerOverSocket {
        self.create_my_data_layer_over_socket()
    }
    fn create_service(
        &self,
        configuration: Configuration,
        my_data_layer_over_socket: MyDataLayerOverSocket,
    ) -> Service {
        Service::new(configuration.port, my_data_layer_over_socket)
    }
    pub fn service(&self) -> Service {
        let configuration = self.configuration();
        let my_data_layer_over_socket = self.my_data_layer_over_socket();
        self.create_service(configuration, my_data_layer_over_socket)
    }
}
fn main() {
    let container = DependencyContainer::new();
    let _service = container.service();
}
