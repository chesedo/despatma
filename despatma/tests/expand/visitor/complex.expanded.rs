/// Complex for all the bells and whistles
use despatma::visitor;
pub trait Visitor {
    fn visit_arc(&mut self, arc: &dyn Arc);
    fn visit_rectangle(&mut self, rectangle: &Rectangle);
    fn visit_point(&mut self, point: &dyn Point) {
        visit_point(self, point)
    }
    fn visit_circle(&mut self, circle: &dyn Circle);
}
pub fn visit_arc<V>(visitor: &mut V, arc: &dyn Arc)
where
    V: Visitor + ?Sized,
{
    visitor.visit_point(arc.center);
}
pub fn visit_rectangle<V>(visitor: &mut V, rectangle: &Rectangle)
where
    V: Visitor + ?Sized,
{
    visitor.visit_point(rectangle.top_left);
    visitor.visit_point(rectangle.bottom_right);
}
pub fn visit_point<V>(_visitor: &mut V, _point: &dyn Point)
where
    V: Visitor + ?Sized,
{}
pub fn visit_circle<V>(_visitor: &mut V, _circle: &dyn Circle)
where
    V: Visitor + ?Sized,
{}
trait Visitable {
    fn apply(&self, visitor: &mut dyn Visitor);
}
impl Visitable for dyn Arc {
    fn apply(&self, visitor: &mut dyn Visitor) {
        visitor.visit_arc(self);
    }
}
impl Visitable for Rectangle {
    fn apply(&self, visitor: &mut dyn Visitor) {
        visitor.visit_rectangle(self);
    }
}
impl Visitable for dyn Point {
    fn apply(&self, visitor: &mut dyn Visitor) {
        visitor.visit_point(self);
    }
}
impl Visitable for dyn Circle {
    fn apply(&self, visitor: &mut dyn Visitor) {
        visitor.visit_circle(self);
    }
}
