/// Test using dyn on a concrete type
mod lib;

use despatma::{abstract_factory, interpolate_traits};
use lib::elements::{Element, Window};

// Factory for a single element
pub trait Factory<T: Element + ?Sized> {
    fn create(&self) -> Box<T>;
}

// Abstract Factory for multiple elements
#[abstract_factory(Factory, dyn Window)]
pub trait AbstractGuiFactory {}

// Concrete factory implementing the abstract factory
struct GnomeFactory {}
impl AbstractGuiFactory for GnomeFactory {}

// Implement the factory trait for each type
#[interpolate_traits(Window => Window)]
impl Factory<dyn TRAIT> for GnomeFactory {
    fn create(&self) -> Box<dyn TRAIT> {
        Box::new(CONCRETE::create())
    }
}

fn main() {}
