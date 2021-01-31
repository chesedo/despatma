/// Simple test for a single non-dynamic factory element
use despatma::{abstract_factory, interpolate_traits};

// Factory for a single element
pub trait Factory<T: Element> {
    fn create(&self, parent: Element) -> T;
}

// Abstract Factory for a single element
#[abstract_factory(Factory, Window)]
pub trait AbstractGuiFactory {}

// Concrete factory implementing the abstract factory
struct QtFactory {}
impl AbstractGuiFactory for QtFactory {}

// Implement the factory trait for each type
#[interpolate_traits(Window => Window)]
impl Factory<TRAIT> for QtFactory {
    fn create(&self, parent: Element) -> TRAIT {
        CONCRETE::new(parent)
    }
}
