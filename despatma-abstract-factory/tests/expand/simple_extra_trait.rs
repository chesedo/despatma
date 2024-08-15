/// Simple test for a single non-dynamic factory element with extra traits
use despatma_abstract_factory::{abstract_factory, interpolate_traits};

// Factory for a single element
pub trait Factory<T: Element> {
    fn create(&self) -> T;
}

// Abstract Factory for single element
#[abstract_factory(Factory, Button)]
pub trait AbstractGuiFactory: Display + Debug {}

// Concrete factory implementing the abstract factory
struct QtFactory {}
impl AbstractGuiFactory for QtFactory {}

// Implement the factory trait for each type
#[interpolate_traits(Window => Window)]
impl Factory<TRAIT> for QtFactory {
    fn create(&self) -> TRAIT {
        CONCRETE::new()
    }
}
