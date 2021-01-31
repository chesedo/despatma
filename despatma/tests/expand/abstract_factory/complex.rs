/// Complex test with all the bells and whistles - mixed types with extra traits
use despatma::{abstract_factory, interpolate_traits};

// Factory for a single shape
pub trait Factory<T: Shape + ?Sized> {
    fn create(&self) -> Box<T>;
}

// Abstract Factory for multiple shapes
#[abstract_factory(Factory, dyn Circle, dyn Rectangle, Arc, dyn Sphere, dyn Cube)]
pub trait AbstractFactory: Display + Eq {}

// Concrete factory implementing the abstract factory
struct RedFactory {}
impl AbstractFactory for RedFactory {}

// Implement the factory trait for each dyn type
#[interpolate_traits(
    Circle => BlueCircle,
    Rectangle => BlueRectangle,
    Sphere => BlueSphere,
    Cube => BlueCube,
)]
impl Factory<dyn TRAIT> for RedFactory {
    fn create(&self) -> Box<dyn TRAIT> {
        Box::new(CONCRETE::new())
    }
}

// Implement the factory trait for each simple type
#[interpolate_traits(
    Arc => Arc,
)]
impl Factory<TRAIT> for RedFactory {
    fn create(&self) -> Box<TRAIT> {
        Box::new(CONCRETE::new())
    }
}
