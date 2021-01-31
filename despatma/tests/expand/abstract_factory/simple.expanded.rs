/// Simple test for a single non-dynamic factory element
use despatma::{abstract_factory, interpolate_traits};
pub trait Factory<T: Element> {
    fn create(&self, parent: Element) -> T;
}
pub trait AbstractGuiFactory: Factory<Window> {}
struct QtFactory {}
impl AbstractGuiFactory for QtFactory {}
impl Factory<Window> for QtFactory {
    fn create(&self, parent: Element) -> Window {
        Window::new(parent)
    }
}
