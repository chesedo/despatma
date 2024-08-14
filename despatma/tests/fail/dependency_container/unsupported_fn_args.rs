struct Config {
    port: u32,
}

struct Service(u32);

struct Task;

#[despatma::dependency_container]
impl DependencyContainer {
    fn config(&self, x!(): u32, [first, ..]: &[i32], [eerste, tweede]: &[i32; 2]) -> Config {
        Config { port: 8080 }
    }

    fn service(&self, Config { port }: Config, (a, b): (i32, i32)) -> Service {
        Service(port)
    }

    fn task(&self, Service(port): Service, _: i32) -> Task {
        Task
    }
}

fn main() {
    let container = DependencyContainer::new();
    let _config = container.config();
}
