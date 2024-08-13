struct Configuration {
    port: u32,
}

struct Task;

impl Task {
    fn new(port: u32) -> Self {
        println!("Task started on port {}", port);
        Self
    }
}

#[despatma::dependency_container]
impl Dependencies {
    fn configuration(&self) -> Configuration {
        Configuration { port: 8080 }
    }

    fn task(&self, configuration: &Configuration) -> Task {
        Task::new(configuration.port)
    }
}

fn main() {
    let deps = Dependencies::new();
    let _task = deps.task();
}
