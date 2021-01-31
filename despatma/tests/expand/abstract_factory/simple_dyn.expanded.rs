/// Simple test for a single dynamic factory element
use despatma::{abstract_factory, interpolate_traits};
pub trait Factory<T: Element + ?Sized> {
    fn create(&self, name: String) -> Box<T>;
}
pub trait AbstractGuiFactory: Factory<dyn Window> {}
struct GnomeFactory {}
impl AbstractGuiFactory for GnomeFactory {}
impl Factory<dyn Window> for GnomeFactory {
    fn create(&self, name: String) -> Box<dyn Window> {
        Box::new(GnomeWindow::new(name))
    }
}
