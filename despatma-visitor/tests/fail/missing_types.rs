/// Test when the types are missing
mod library;

use despatma_visitor::visitor;

visitor!(dyn Circle, Rectangle, dyn Sphere, Arc, dyn Cube);

fn main() {}
