//! This test creates a multiline function arg to ensure trailing commas are handled correctly

struct Configuration {
    port: u32,
}

struct MyDataLayerOverSocket;

struct Service;

impl Service {
    fn new(port: u32, _my_data_layer_over_socket: MyDataLayerOverSocket) -> Self {
        println!("Trailing comma service started on port {}", port);
        Self
    }
}

#[despatma_dependency_container::dependency_container]
impl DependencyContainer {
    fn configuration(&self) -> Configuration {
        Configuration { port: 8080 }
    }

    fn my_data_layer_over_socket(&self) -> MyDataLayerOverSocket {
        MyDataLayerOverSocket
    }

    fn service(
        &self,
        configuration: Configuration,
        my_data_layer_over_socket: MyDataLayerOverSocket,
    ) -> Service {
        Service::new(configuration.port, my_data_layer_over_socket)
    }
}

fn main() {
    let container = DependencyContainer::new();
    let _service = container.service();
}
