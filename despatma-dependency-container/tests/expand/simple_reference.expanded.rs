struct Configuration {
    port: u32,
}
struct Task;
impl Task {
    fn new(port: u32) -> Self {
        {
            ::std::io::_print(format_args!("Task started on port {0}\n", port));
        };
        Self
    }
}
struct Dependencies;
impl Dependencies {
    pub fn new() -> Self {
        Self
    }
    pub fn new_scope(&self) -> Self {
        Self
    }
    fn create_configuration(&self) -> Configuration {
        Configuration { port: 8080 }
    }
    pub fn configuration(&self) -> Configuration {
        self.create_configuration()
    }
    fn create_task(&self, configuration: &Configuration) -> Task {
        Task::new(configuration.port)
    }
    pub fn task(&self) -> Task {
        let configuration = self.configuration();
        self.create_task(&configuration)
    }
}
fn main() {
    let deps = Dependencies::new();
    let _task = deps.task();
}
