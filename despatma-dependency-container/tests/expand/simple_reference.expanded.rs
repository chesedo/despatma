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
struct Dependencies<'a> {
    _phantom: std::marker::PhantomData<&'a ()>,
}
#[automatically_derived]
impl<'a> ::core::clone::Clone for Dependencies<'a> {
    #[inline]
    fn clone(&self) -> Dependencies<'a> {
        Dependencies {
            _phantom: ::core::clone::Clone::clone(&self._phantom),
        }
    }
}
impl<'a> Dependencies<'a> {
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
    pub fn configuration(&'a self) -> Configuration {
        Configuration { port: 8080 }
    }
    pub fn task(&'a self) -> Task {
        let configuration = Configuration { port: 8080 };
        Task::new(configuration.port)
    }
}
fn main() {
    let deps = Dependencies::new();
    let _task = deps.task();
}
