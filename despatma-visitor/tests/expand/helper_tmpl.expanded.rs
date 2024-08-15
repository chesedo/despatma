/// Test for the no_default option
use despatma_visitor::visitor;
pub trait Visitor {
    fn visit_arc(&mut self, arc: &dyn Arc) {
        visit_arc(self, arc)
    }
    fn visit_rectangle(&mut self, rectangle: &Rectangle) {
        visit_rectangle(self, rectangle)
    }
    fn visit_point(&mut self, point: &Point) {
        visit_point(self, point)
    }
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
    visitor.visit_point(&rectangle.top_left);
    visitor.visit_point(&rectangle.bottom_right);
}
pub fn visit_point<V>(_visitor: &mut V, _point: &Point)
where
    V: Visitor + ?Sized,
{}
trait Visitable {
    fn apply(&self, visitor: &mut impl Visitor);
}
impl Visitable for dyn Arc {
    fn apply(&self, visitor: &mut impl Visitor) {
        visitor.visit_arc(self);
    }
}
impl Visitable for Rectangle {
    fn apply(&self, visitor: &mut impl Visitor) {
        visitor.visit_rectangle(self);
    }
}
impl Visitable for Point {
    fn apply(&self, visitor: &mut impl Visitor) {
        visitor.visit_point(self);
    }
}
