/// All gui elements
pub trait Element {
    fn create() -> Self
    where
        Self: Sized;
}

/// Non-abstract window element to test not using `dyn`
pub struct Window {
    width: usize,
}

impl Element for Window {
    fn create() -> Self {
        Window { width: 50 }
    }
}

/// Abstract type
pub trait Button: Element {
    fn click(&self);
}
