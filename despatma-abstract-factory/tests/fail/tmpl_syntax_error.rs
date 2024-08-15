/// Test syntax error in the interpolation template
mod library;

use despatma_abstract_factory::{abstract_factory, interpolate_traits};
use library::elements::{Button, Element, Window};

struct GnomeButton {}

impl Element for GnomeButton {
    fn create() -> Self {
        GnomeButton {}
    }
}
impl Button for GnomeButton {
    fn click(&self) {}
}

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

// Implement the factory trait for each type
#[interpolate_traits(Button => GnomeButton)]
impl Factory<dyn TRAIT> for GnomeFactory {
    fn create(&self) -> Box<dyn TRAIT> {
        Box::new(CONCRETE::new())
    }
}
#[interpolate_traits(Window => Window)]
impl Factory<TRAIT> for GnomeFactory {
    fn create(&self) -> Box<TRAIT> {
        Box::new(CONCRETE::create());
    }
}

fn main() {}
