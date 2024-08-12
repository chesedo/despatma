/// Test for the no_default option
use despatma::visitor;
pub trait Visitor {
    fn visit_button(&mut self, button: &dyn Button);
    fn visit_window(&mut self, window: &Window);
}
pub fn visit_button<V>(_visitor: &mut V, _button: &dyn Button)
where
    V: Visitor + ?Sized,
{}
pub fn visit_window<V>(_visitor: &mut V, _window: &Window)
where
    V: Visitor + ?Sized,
{}
trait Visitable {
    fn apply(&self, visitor: &mut impl Visitor);
}
impl Visitable for dyn Button {
    fn apply(&self, visitor: &mut impl Visitor) {
        visitor.visit_button(self);
    }
}
impl Visitable for Window {
    fn apply(&self, visitor: &mut impl Visitor) {
        visitor.visit_window(self);
    }
}
