/// Test a syntax error in the helper_tmpl option
mod lib;

use despatma::visitor;
use lib::shapes::{Arc, Circle, Cube};

visitor!(
    #[no_default]
    dyn Circle,

    Arc,

    #[helper_tmpl = {visitor.visit_rectangle(cube.get_back())}]
    dyn Cube,
);

fn main() {}
