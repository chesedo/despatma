struct Config {
    port: u32,
}

#[despatma_dependency_container::dependency_container]
impl DependencyContainer {
    const SIZE: usize = 1;

    type DB = Sqlite;

    db!();

    fn config(&self) -> Config {
        Config { port: 8080 }
    }
}

fn main() {
    let container = DependencyContainer::new();
    let _config = container.config();
}
