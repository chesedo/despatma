/// Test for the no_default option
use despatma::visitor;

visitor!(
    #[helper_tmpl = {visitor.visit_point(arc.center);}]
    dyn Arc,

    #[helper_tmpl = {
        visitor.visit_point(&rectangle.top_left);
        visitor.visit_point(&rectangle.bottom_right);
    }]
    Rectangle,

    Point,
);
