/// Simple test for a single element
use despatma_visitor::visitor;
pub trait Visitor {
    fn visit_window(&mut self, window: &Window) {
        visit_window(self, window)
    }
}
pub fn visit_window<V>(_visitor: &mut V, _window: &Window)
where
    V: Visitor + ?Sized,
{}
trait Visitable {
    fn apply(&self, visitor: &mut impl Visitor);
}
impl Visitable for Window {
    fn apply(&self, visitor: &mut impl Visitor) {
        visitor.visit_window(self);
    }
}
