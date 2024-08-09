/// Simple test for multiple non-dynamic factory elements
use despatma::{abstract_factory, interpolate_traits};
pub trait Builder<T: ElementBuilder> {
    fn create(&self) -> T;
}
pub trait AbstractBuilders: Builder<
        Window,
    > + Builder<Button> + Builder<Scroller> + Builder<CheckBox> + Builder<RadioBox> {}
struct QtBuilders {}
impl AbstractBuilders for QtBuilders {}
impl Builder<Window> for QtBuilders {
    fn create(&self) -> Window {
        QtWindowBuilder::new()
    }
}
impl Builder<Button> for QtBuilders {
    fn create(&self) -> Button {
        QtButtonBuilder::new()
    }
}
impl Builder<Scroller> for QtBuilders {
    fn create(&self) -> Scroller {
        QtScrollerBuilder::new()
    }
}
impl Builder<CheckBox> for QtBuilders {
    fn create(&self) -> CheckBox {
        QtCheckBoxBuilder::new()
    }
}
impl Builder<RadionBox> for QtBuilders {
    fn create(&self) -> RadionBox {
        QtRadionBoxBuilder::new()
    }
}
