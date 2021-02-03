/// Test using two attributes to provide options
mod lib;

use despatma::visitor;
use lib::shapes::{Circle, Cube, Rectangle};

visitor!(
    #[no_default]
    dyn Circle,

    Rectangle,

    #[helper_tmpl = {visitor.visit_rectangle(cube.get_front())}]
    #[no_default]
    dyn Cube,
);

fn main() {}
