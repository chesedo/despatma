use auto_impl::auto_impl;
use despatma::dependency_container;
use std::{fmt::Display, sync::Arc};
use tokio::sync::Mutex;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

// Entities Layer
#[derive(Clone)]
struct User {
    id: u32,
    name: String,
    email: String,
    age: u8,
}

// Use Cases Layer (Core Business Logic)
#[auto_impl(&)]
trait UserRepository: Send + Sync {
    async fn get_user(&self, id: u32) -> Option<User>;
    async fn save_user(&self, user: &User) -> Result<(), String>;
}

struct UserService<R> {
    repository: R,
}

impl<R: UserRepository> UserService<R> {
    fn new(repository: R) -> Self {
        Self { repository }
    }

    async fn get_user(&self, id: u32) -> Result<User, String> {
        self.repository
            .get_user(id)
            .await
            .ok_or_else(|| format!("User with id {} not found", id))
    }

    async fn create_user(&self, name: String, email: String, age: u8) -> Result<User, String> {
        // Core business logic: User creation rules
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if !email.contains('@') {
            return Err("Invalid email format".to_string());
        }
        if age < 18 {
            return Err("User must be at least 18 years old".to_string());
        }

        let user = User {
            id: 0, // Assume ID is assigned by the repository
            name,
            email,
            age,
        };

        self.repository.save_user(&user).await.map(|_| user)
    }
}

// Interface Adapters Layer
trait UserPresenter: Send + Sync {
    fn present_user(&self, user: &User) -> String;
    fn present_error(&self, error: &str) -> String;
}

struct UserController<R, P> {
    user_service: UserService<R>,
    presenter: P,
}

impl<R: UserRepository, P: UserPresenter> UserController<R, P> {
    fn new(user_service: UserService<R>, presenter: P) -> Self {
        Self {
            user_service,
            presenter,
        }
    }

    async fn get_user(&self, id: u32) -> String {
        match self.user_service.get_user(id).await {
            Ok(user) => self.presenter.present_user(&user),
            Err(e) => self.presenter.present_error(&e),
        }
    }

    async fn create_user(&self, name: String, email: String, age: u8) -> String {
        match self.user_service.create_user(name, email, age).await {
            Ok(user) => self.presenter.present_user(&user),
            Err(e) => self.presenter.present_error(&e),
        }
    }
}

// Frameworks & Drivers Layer
struct InMemoryUserRepository {
    users: Mutex<Vec<User>>,
}

impl InMemoryUserRepository {
    fn new() -> Self {
        Self {
            users: Mutex::new(Vec::new()),
        }
    }
}

impl UserRepository for InMemoryUserRepository {
    async fn get_user(&self, id: u32) -> Option<User> {
        let users = self.users.lock().await;
        users.iter().find(|u| u.id == id).cloned()
    }

    async fn save_user(&self, user: &User) -> Result<(), String> {
        let mut users = self.users.lock().await;
        let id = users.len() as u32 + 1;
        let mut new_user = user.clone();
        new_user.id = id;
        users.push(new_user);
        Ok(())
    }
}

struct OneLinePresenter;

impl UserPresenter for OneLinePresenter {
    fn present_user(&self, user: &User) -> String {
        format!(
            "User {}: {} ({}, age {})",
            user.id, user.name, user.email, user.age
        )
    }

    fn present_error(&self, error: &str) -> String {
        format!("Error: {}", error)
    }
}

enum Action {
    Get,
    Create,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Get => write!(f, "Get"),
            Action::Create => write!(f, "Create"),
        }
    }
}

struct WebFramework<R, P> {
    user_controller: Arc<UserController<R, P>>,
    request_count: Arc<Mutex<u32>>,
}

impl<R: UserRepository, P: UserPresenter> WebFramework<R, P> {
    fn new(user_controller: Arc<UserController<R, P>>, request_count: Arc<Mutex<u32>>) -> Self {
        Self {
            user_controller,
            request_count,
        }
    }

    async fn handle_request(&self, action: Action, params: &[String]) -> String {
        let count = {
            let mut rc = self.request_count.lock().await;
            *rc += 1;
            *rc
        };

        let response = match action {
            Action::Get => {
                if let Some(id) = params.first().and_then(|s| s.parse().ok()) {
                    self.user_controller.get_user(id).await
                } else {
                    "Invalid user ID".to_string()
                }
            }
            Action::Create => {
                if params.len() == 3 {
                    if let Ok(age) = params[2].parse() {
                        self.user_controller
                            .create_user(params[0].clone(), params[1].clone(), age)
                            .await
                    } else {
                        "Invalid age".to_string()
                    }
                } else {
                    "Invalid parameters for user creation".to_string()
                }
            }
        };

        info!(request_number = count, action = %action, "Handled request");
        format!("{} (Request #{})", response, count)
    }
}

// Dependency Container
#[dependency_container]
impl AppContainer {
    #[Singleton(InMemoryUserRepository)]
    async fn user_repository(&self) -> impl UserRepository {
        InMemoryUserRepository::new()
    }

    fn user_service(
        &self,
        user_repository: impl UserRepository,
    ) -> UserService<impl UserRepository> {
        UserService::new(user_repository)
    }

    #[Transient(OneLinePresenter)]
    fn user_presenter(&self) -> impl UserPresenter {
        OneLinePresenter
    }

    #[Singleton]
    fn user_controller(
        &self,
        user_service: UserService<impl UserRepository>,
        user_presenter: impl UserPresenter,
    ) -> Arc<UserController<impl UserRepository, impl UserPresenter>> {
        Arc::new(UserController::new(user_service, user_presenter))
    }

    /// Just setup tracing and be done with it
    #[Singleton]
    fn _tracing(&self) -> () {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to set tracing subscriber");
    }

    #[Singleton]
    fn request_count(&self) -> Arc<Mutex<u32>> {
        Arc::new(Mutex::new(0))
    }

    fn web_framework(
        &self,
        _tracing: (),
        user_controller: &Arc<UserController<impl UserRepository, impl UserPresenter>>,
        request_count: &Arc<Mutex<u32>>,
    ) -> WebFramework<impl UserRepository, impl UserPresenter> {
        WebFramework::new(user_controller.clone(), request_count.clone())
    }
}

#[tokio::main]
async fn main() {
    let container = AppContainer::new();
    let web_framework = container.web_framework().await;

    // Simulate API requests
    println!(
        "{}",
        web_framework
            .handle_request(
                Action::Create,
                &[
                    "Alice".to_string(),
                    "alice@example.com".to_string(),
                    "30".to_string()
                ]
            )
            .await
    );
    println!(
        "{}",
        web_framework
            .handle_request(Action::Get, &["1".to_string()])
            .await
    );
    println!(
        "{}",
        web_framework
            .handle_request(
                Action::Create,
                &[
                    "Bob".to_string(),
                    "bob@example.com".to_string(),
                    "17".to_string()
                ]
            )
            .await
    );
}
