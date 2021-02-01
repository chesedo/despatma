/// Test for the no_default option
use despatma::visitor;

visitor!(
    #[no_default]
    dyn Button,

    #[no_default]
    Window,
);
