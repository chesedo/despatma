/// Test when `dyn`s are used wrong
mod lib;

use despatma::visitor;
use lib::shapes::{Arc, Circle, Cube, Rectangle, Sphere};

visitor!(dyn Circle, dyn Rectangle, dyn Sphere, Arc, Cube);

fn main() {}
