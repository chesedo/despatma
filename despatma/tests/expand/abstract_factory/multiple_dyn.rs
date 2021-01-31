/// Simple test for multiple dynamic factory elements
use despatma::{abstract_factory, interpolate_traits};

// Factory for a single shape
pub trait Factory<T: Shape + ?Sized> {
    fn create(&self) -> Box<T>;
}

// Abstract Factory for multiple shapes
#[abstract_factory(Factory, dyn Circle, dyn Rectangle, dyn Arc, dyn Sphere, dyn Cube)]
pub trait AbstractFactory {}

// Concrete factory implementing the abstract factory
struct BlueFactory {}
impl AbstractFactory for BlueFactory {}

// Implement the factory trait for each type
#[interpolate_traits(
    Circle => BlueCircle,
    Rectangle => BlueRectangle,
    Arc => BlueArc,
    Sphere => BlueSphere,
    Cube => BlueCube,
)]
impl Factory<dyn TRAIT> for BlueFactory {
    fn create(&self) -> Box<dyn TRAIT> {
        Box::new(CONCRETE::new())
    }
}
