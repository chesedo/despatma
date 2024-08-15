/// Simple test for multiple dynamic factory elements
use despatma_abstract_factory::{abstract_factory, interpolate_traits};
pub trait Factory<T: Shape + ?Sized> {
    fn create(&self) -> Box<T>;
}
pub trait AbstractFactory: Factory<
        dyn Circle,
    > + Factory<
        dyn Rectangle,
    > + Factory<dyn Arc> + Factory<dyn Sphere> + Factory<dyn Cube> {}
struct BlueFactory {}
impl AbstractFactory for BlueFactory {}
impl Factory<dyn Circle> for BlueFactory {
    fn create(&self) -> Box<dyn Circle> {
        Box::new(BlueCircle::new())
    }
}
impl Factory<dyn Rectangle> for BlueFactory {
    fn create(&self) -> Box<dyn Rectangle> {
        Box::new(BlueRectangle::new())
    }
}
impl Factory<dyn Arc> for BlueFactory {
    fn create(&self) -> Box<dyn Arc> {
        Box::new(BlueArc::new())
    }
}
impl Factory<dyn Sphere> for BlueFactory {
    fn create(&self) -> Box<dyn Sphere> {
        Box::new(BlueSphere::new())
    }
}
impl Factory<dyn Cube> for BlueFactory {
    fn create(&self) -> Box<dyn Cube> {
        Box::new(BlueCube::new())
    }
}
