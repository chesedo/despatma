/// Complex test with all the bells and whistles - mixed types with extra traits
use despatma::{abstract_factory, interpolate_traits};
pub trait Factory<T: Shape + ?Sized> {
    fn create(&self) -> Box<T>;
}
pub trait AbstractFactory: Display + Eq + Factory<
        dyn Circle,
    > + Factory<dyn Rectangle> + Factory<Arc> + Factory<dyn Sphere> + Factory<dyn Cube> {}
struct RedFactory {}
impl AbstractFactory for RedFactory {}
impl Factory<dyn Circle> for RedFactory {
    fn create(&self) -> Box<dyn Circle> {
        Box::new(BlueCircle::new())
    }
}
impl Factory<dyn Rectangle> for RedFactory {
    fn create(&self) -> Box<dyn Rectangle> {
        Box::new(BlueRectangle::new())
    }
}
impl Factory<dyn Sphere> for RedFactory {
    fn create(&self) -> Box<dyn Sphere> {
        Box::new(BlueSphere::new())
    }
}
impl Factory<dyn Cube> for RedFactory {
    fn create(&self) -> Box<dyn Cube> {
        Box::new(BlueCube::new())
    }
}
impl Factory<Arc> for RedFactory {
    fn create(&self) -> Box<Arc> {
        Box::new(Arc::new())
    }
}
