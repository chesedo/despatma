use auto_impl::auto_impl;
use despatma::dependency_container;

// Entities Layer
#[derive(Clone)]
struct User {
    name: String,
}

// Use Cases Layer
#[auto_impl(&)]
trait UserRepository {
    fn get_user(&self, id: u32) -> Option<User>;
}

struct GreetUseCase<R> {
    user_repository: R,
}

impl<R: UserRepository> GreetUseCase<R> {
    fn new(user_repository: R) -> Self {
        Self { user_repository }
    }

    fn greet(&self, user_id: u32) -> String {
        match self.user_repository.get_user(user_id) {
            Some(user) => format!("Hello, {}!", user.name),
            None => "Hello, Guest!".to_string(),
        }
    }
}

// Interface Adapters Layer
struct ConsolePresenter;

impl ConsolePresenter {
    fn present(&self, message: &str) {
        println!("{}", message);
    }
}

// Frameworks & Drivers Layer
struct InMemoryUserRepository {
    users: Vec<User>,
}

impl InMemoryUserRepository {
    fn new() -> Self {
        Self {
            users: vec![
                User {
                    name: "Alice".to_string(),
                },
                User {
                    name: "Bob".to_string(),
                },
            ],
        }
    }
}

impl UserRepository for InMemoryUserRepository {
    fn get_user(&self, id: u32) -> Option<User> {
        self.users.get(id as usize).cloned()
    }
}

// Dependency Container
#[dependency_container]
impl AppContainer {
    #[Singleton(InMemoryUserRepository)]
    fn user_repository(&self) -> impl UserRepository {
        InMemoryUserRepository::new()
    }

    #[Transient]
    fn greet_use_case(
        &self,
        user_repository: impl UserRepository,
    ) -> GreetUseCase<impl UserRepository> {
        GreetUseCase::new(user_repository)
    }

    #[Singleton]
    fn console_presenter(&self) -> ConsolePresenter {
        ConsolePresenter
    }
}

fn main() {
    let container = AppContainer::new();

    let use_case = container.greet_use_case();
    let presenter = container.console_presenter();

    // Greet existing user
    let greeting = use_case.greet(0);
    presenter.present(&greeting);

    // Greet non-existing user
    let greeting = use_case.greet(5);
    presenter.present(&greeting);
}
