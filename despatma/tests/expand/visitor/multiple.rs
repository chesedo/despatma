/// Test with multiple visitable elements
use despatma::visitor;

visitor!(dyn Circle, Rectangle, dyn Sphere, Arc, dyn Cube,);
