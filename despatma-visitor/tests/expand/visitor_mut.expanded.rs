/// Complex for all the bells and whistles for visitor that mutates the elements
use despatma_visitor::visitor_mut;
pub trait VisitorMut {
    fn visit_arc_mut(&mut self, arc: &mut dyn Arc);
    fn visit_rectangle_mut(&mut self, rectangle: &mut Rectangle);
    fn visit_point_mut(&mut self, point: &mut dyn Point) {
        visit_point_mut(self, point)
    }
    fn visit_circle_mut(&mut self, circle: &mut dyn Circle);
}
pub fn visit_arc_mut<V>(visitor: &mut V, arc: &mut dyn Arc)
where
    V: VisitorMut + ?Sized,
{
    visitor.visit_point_mut(&arc.center);
}
pub fn visit_rectangle_mut<V>(visitor: &mut V, rectangle: &mut Rectangle)
where
    V: VisitorMut + ?Sized,
{
    visitor.visit_point_mut(&rectangle.top_left);
    visitor.visit_point_mut(&rectangle.bottom_right);
}
pub fn visit_point_mut<V>(_visitor: &mut V, _point: &mut dyn Point)
where
    V: VisitorMut + ?Sized,
{}
pub fn visit_circle_mut<V>(_visitor: &mut V, _circle: &mut dyn Circle)
where
    V: VisitorMut + ?Sized,
{}
pub trait VisitableMut {
    fn apply_mut(&mut self, visitor: &mut impl VisitorMut);
}
impl VisitableMut for dyn Arc {
    fn apply_mut(&mut self, visitor: &mut impl VisitorMut) {
        visitor.visit_arc_mut(self);
    }
}
impl VisitableMut for Rectangle {
    fn apply_mut(&mut self, visitor: &mut impl VisitorMut) {
        visitor.visit_rectangle_mut(self);
    }
}
impl VisitableMut for dyn Point {
    fn apply_mut(&mut self, visitor: &mut impl VisitorMut) {
        visitor.visit_point_mut(self);
    }
}
impl VisitableMut for dyn Circle {
    fn apply_mut(&mut self, visitor: &mut impl VisitorMut) {
        visitor.visit_circle_mut(self);
    }
}
