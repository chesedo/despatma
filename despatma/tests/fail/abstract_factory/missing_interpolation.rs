/// Test when concrete trait interpolations are missing
mod library;

use despatma::abstract_factory;
use library::elements::{Button, Element, Window};

// Factory for a single element
pub trait Factory<T: Element + ?Sized> {
    fn create(&self) -> Box<T>;
}

// Abstract Factory for multiple elements
#[abstract_factory(Factory, dyn Button, Window)]
pub trait AbstractGuiFactory {}

// Concrete factory implementing the abstract factory
struct GnomeFactory {}
impl AbstractGuiFactory for GnomeFactory {}

fn main() {}
