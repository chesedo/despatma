/// Simple test for multiple non-dynamic factory elements
use despatma_abstract_factory::{abstract_factory, interpolate_traits};

// Factory (Builder) for a single element
pub trait Builder<T: ElementBuilder> {
    fn create(&self) -> T;
}

// Abstract Factory (Builders) for multiple elements
#[abstract_factory(Builder, Window, Button, Scroller, CheckBox, RadioBox)]
pub trait AbstractBuilders {}

// Concrete factory (builder) implementing the abstract factory (builder)
struct QtBuilders {}
impl AbstractBuilders for QtBuilders {}

// Implement the factory trait for each type
#[interpolate_traits(
    Window => QtWindowBuilder,
    Button => QtButtonBuilder,
    Scroller => QtScrollerBuilder,
    CheckBox => QtCheckBoxBuilder,
    RadionBox => QtRadionBoxBuilder,
)]
impl Builder<TRAIT> for QtBuilders {
    fn create(&self) -> TRAIT {
        CONCRETE::new()
    }
}
