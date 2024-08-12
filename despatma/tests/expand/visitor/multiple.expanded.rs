/// Test with multiple visitable elements
use despatma::visitor;
pub trait Visitor {
    fn visit_circle(&mut self, circle: &dyn Circle) {
        visit_circle(self, circle)
    }
    fn visit_rectangle(&mut self, rectangle: &Rectangle) {
        visit_rectangle(self, rectangle)
    }
    fn visit_sphere(&mut self, sphere: &dyn Sphere) {
        visit_sphere(self, sphere)
    }
    fn visit_arc(&mut self, arc: &Arc) {
        visit_arc(self, arc)
    }
    fn visit_cube(&mut self, cube: &dyn Cube) {
        visit_cube(self, cube)
    }
}
pub fn visit_circle<V>(_visitor: &mut V, _circle: &dyn Circle)
where
    V: Visitor + ?Sized,
{}
pub fn visit_rectangle<V>(_visitor: &mut V, _rectangle: &Rectangle)
where
    V: Visitor + ?Sized,
{}
pub fn visit_sphere<V>(_visitor: &mut V, _sphere: &dyn Sphere)
where
    V: Visitor + ?Sized,
{}
pub fn visit_arc<V>(_visitor: &mut V, _arc: &Arc)
where
    V: Visitor + ?Sized,
{}
pub fn visit_cube<V>(_visitor: &mut V, _cube: &dyn Cube)
where
    V: Visitor + ?Sized,
{}
trait Visitable {
    fn apply(&self, visitor: &mut impl Visitor);
}
impl Visitable for dyn Circle {
    fn apply(&self, visitor: &mut impl Visitor) {
        visitor.visit_circle(self);
    }
}
impl Visitable for Rectangle {
    fn apply(&self, visitor: &mut impl Visitor) {
        visitor.visit_rectangle(self);
    }
}
impl Visitable for dyn Sphere {
    fn apply(&self, visitor: &mut impl Visitor) {
        visitor.visit_sphere(self);
    }
}
impl Visitable for Arc {
    fn apply(&self, visitor: &mut impl Visitor) {
        visitor.visit_arc(self);
    }
}
impl Visitable for dyn Cube {
    fn apply(&self, visitor: &mut impl Visitor) {
        visitor.visit_cube(self);
    }
}
