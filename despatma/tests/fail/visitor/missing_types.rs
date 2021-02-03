/// Test when the types are missing
mod lib;

use despatma::visitor;

visitor!(dyn Circle, Rectangle, dyn Sphere, Arc, dyn Cube);

fn main() {}
