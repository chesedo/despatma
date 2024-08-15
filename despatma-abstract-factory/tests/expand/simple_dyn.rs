/// Simple test for a single dynamic factory element
use despatma_abstract_factory::{abstract_factory, interpolate_traits};

// Factory for a single element
pub trait Factory<T: Element + ?Sized> {
    fn create(&self, name: String) -> Box<T>;
}

// Abstract Factory for multiple elements
#[abstract_factory(Factory, dyn Window)]
pub trait AbstractGuiFactory {}

// Concrete factory implementing the abstract factory
struct GnomeFactory {}
impl AbstractGuiFactory for GnomeFactory {}

// Implement the factory trait for each type
#[interpolate_traits(Window => GnomeWindow)]
impl Factory<dyn TRAIT> for GnomeFactory {
    fn create(&self, name: String) -> Box<dyn TRAIT> {
        Box::new(CONCRETE::new(name))
    }
}
