use crate::container::{ChildDependency, Container, Dependency};

mod check_wiring;
mod fix_async_tree;
mod impl_trait_but_registered_concrete;
mod set_output_spans;

pub use check_wiring::CheckWiring;
use despatma_visitor::{visitor, visitor_mut};
pub use fix_async_tree::FixAsyncTree;
pub use impl_trait_but_registered_concrete::ImplTraitButRegisteredConcrete;
pub use set_output_spans::SetOutputSpans;

visitor!(
    #[helper_tmpl = {
        for dependency in container.dependencies.values() {
            visitor.visit_dependency(&dependency.borrow());
        }
    }]
    Container,
    #[helper_tmpl = {
        for child_dependency in &dependency.dependencies {
            visitor.visit_child_dependency(child_dependency);
        }
    }]
    Dependency,
    ChildDependency,
);

/// A visitor used to validate the struct that will be turned into a dependency container.
/// If the visitor found any errors then they should be emit in [emit_errors].
pub trait ErrorVisitor: Visitor {
    fn emit_errors(self);
}

// A mutable visitor used to update any dependencies or their children
visitor_mut!(
    #[helper_tmpl = {
        for dependency in container.dependencies.values_mut() {
            visitor.visit_dependency_mut(&mut dependency.borrow_mut());
        }
    }]
    Container,
    #[helper_tmpl = {
        for child_dependency in &mut dependency.dependencies {
            visitor.visit_child_dependency_mut(child_dependency);
        }
    }]
    Dependency,
    ChildDependency,
);
