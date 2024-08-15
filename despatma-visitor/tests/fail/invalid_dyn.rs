/// Test when `dyn`s are used wrong
mod library;

use despatma_visitor::visitor;
use library::shapes::{Arc, Circle, Cube, Rectangle, Sphere};

visitor!(dyn Circle, dyn Rectangle, dyn Sphere, Arc, Cube);

fn main() {}
