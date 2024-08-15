/// Test an element with a compound name
use despatma_visitor::visitor;
pub trait Visitor {
    fn visit_gnome_3_window(&mut self, gnome_3_window: &Gnome3Window) {
        visit_gnome_3_window(self, gnome_3_window)
    }
}
pub fn visit_gnome_3_window<V>(_visitor: &mut V, _gnome_3_window: &Gnome3Window)
where
    V: Visitor + ?Sized,
{}
trait Visitable {
    fn apply(&self, visitor: &mut impl Visitor);
}
impl Visitable for Gnome3Window {
    fn apply(&self, visitor: &mut impl Visitor) {
        visitor.visit_gnome_3_window(self);
    }
}
