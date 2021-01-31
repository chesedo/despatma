/// Simple test for a single non-dynamic factory element with extra traits
use despatma::{abstract_factory, interpolate_traits};
pub trait Factory<T: Element> {
    fn create(&self) -> T;
}
pub trait AbstractGuiFactory: Display + Debug + Factory<Button> {}
struct QtFactory {}
impl AbstractGuiFactory for QtFactory {}
impl Factory<Window> for QtFactory {
    fn create(&self) -> Window {
        Window::new()
    }
}
