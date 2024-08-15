/// Test with multiple visitable elements
use despatma_visitor::visitor;

visitor!(dyn Circle, Rectangle, dyn Sphere, Arc, dyn Cube,);
