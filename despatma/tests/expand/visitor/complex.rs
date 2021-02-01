/// Complex for all the bells and whistles
use despatma::visitor;

visitor!(
    #[
        helper_tmpl = {visitor.visit_point(arc.center);},
        no_default,
    ]
    dyn Arc,

    #[
        no_default,
        helper_tmpl = {
            visitor.visit_point(rectangle.top_left);
            visitor.visit_point(rectangle.bottom_right);
        },
    ]
    Rectangle,

    dyn Point,

    #[no_default]
    dyn Circle,
);
