/// Complex for all the bells and whistles for visitor that mutates the elements
use despatma_visitor::visitor_mut;

visitor_mut!(
    #[
        helper_tmpl = {visitor.visit_point_mut(&arc.center);},
        no_default,
    ]
    dyn Arc,

    #[
        no_default,
        helper_tmpl = {
            visitor.visit_point_mut(&rectangle.top_left);
            visitor.visit_point_mut(&rectangle.bottom_right);
        },
    ]
    Rectangle,

    dyn Point,

    #[no_default]
    dyn Circle,
);
