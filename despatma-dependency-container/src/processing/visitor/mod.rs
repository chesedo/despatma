use despatma_visitor::visitor_mut;

use super::{ChildDependency, Container, Dependency};

pub use add_wildcard_lifetime::AddWildcardLifetime;
pub use extract_async::ExtractAsync;
pub use extract_box_type::ExtractBoxType;
pub use extract_lifetime::ExtractLifetime;
pub use impl_trait_but_registered_concrete::ImplTraitButRegisteredConcrete;
pub use impl_trait_fields::ImplTraitFields;
pub use link_dependencies::LinkDependencies;
pub use owning_managed_dependency::OwningManagedDependency;
pub use replace_impl_generics_with_concrete::ReplaceImplGenericsWithConcrete;
pub use unsupported_registered_types::UnsupportedRegisteredTypes;
pub use wrap_box_type::WrapBoxType;

mod add_wildcard_lifetime;
mod extract_async;
mod extract_box_type;
mod extract_lifetime;
mod impl_trait_but_registered_concrete;
mod impl_trait_fields;
mod link_dependencies;
mod owning_managed_dependency;
mod replace_impl_generics_with_concrete;
mod unsupported_registered_types;
mod wrap_box_type;

visitor_mut!(
    #[helper_tmpl = {
        for dependency in container.dependencies.iter() {
            visitor.visit_dependency_mut(&mut dependency.borrow_mut());
        }
    }]
    Container,
    #[helper_tmpl = {
        for dependency in dependency.dependencies.iter_mut() {
            visitor.visit_child_dependency_mut(dependency);
        }
    }]
    Dependency,
    #[helper_tmpl = {
        visitor.visit_dependency_mut(&mut child_dependency.inner.borrow_mut());
    }]
    ChildDependency,
);

/// A visitor used for validation
/// If the visitor found any errors then they should be emit in [emit_errors].
pub trait ErrorVisitorMut: VisitorMut + Sized {
    fn new() -> Self;

    fn emit_errors(self) {}
}
