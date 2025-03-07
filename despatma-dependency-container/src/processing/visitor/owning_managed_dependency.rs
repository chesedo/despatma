use proc_macro_error2::emit_error;
use quote::ToTokens;
use syn::Type;

use crate::processing::Dependency;

use super::{ErrorVisitorMut, VisitorMut};

/// Visitor to help users when they are trying to take ownership of a dependency which is managed by the container.
/// The container needs to manage Singleton and Scoped dependencies to control their constructions. This means references
/// of theses dependencies are given to anything that requires them. Another visitor already fixes this for `impl Trait`
/// dependencies. This visitor fixes it for concrete (non-abstract) dependencies.
///
/// Needs to happen after child dependencies are linked.
/// And after lifetimes are extracted.
/// But before any types are changed.
pub struct OwningManagedDependency {
    types: Vec<Type>,
}

impl VisitorMut for OwningManagedDependency {
    fn visit_dependency_mut(&mut self, dependency: &mut Dependency) {
        let types = dependency
            .dependencies
            .iter()
            .filter(|child| matches!(child.ty, Type::Path(_)))
            .filter(|child| child.inner.borrow().lifetime.is_managed())
            .map(|child| child.ty.clone());

        self.types.extend(types);
    }
}

impl ErrorVisitorMut for OwningManagedDependency {
    fn new() -> Self {
        Self {
            types: Default::default(),
        }
    }

    fn emit_errors(self) {
        for ty in self.types {
            emit_error!(
                ty, "This dependency is managed by the container";
                hint = "Take a reference here instead: &{}",  ty.to_token_stream();
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use syn::parse_quote;

    use crate::{
        input,
        processing::{
            self,
            visitor::{ExtractLifetime, LinkDependencies, VisitableMut},
        },
    };

    use super::*;

    #[test]
    fn owning_managed_dependency() {
        let mut container: processing::Container = input::Container::from_item_impl(parse_quote!(
            impl Container {
                #[Singleton]
                fn config(&self) -> Config {
                    Config
                }

                fn service(&self, config: Config) -> Service {
                    Service::new(config)
                }
            }
        ))
        .into();

        // Test needs them to be linked nd lifetimes to be extracted
        container.apply_mut(&mut LinkDependencies::new());
        container.apply_mut(&mut ExtractLifetime::new());

        let mut visitor = OwningManagedDependency::new();

        container.apply_mut(&mut visitor);

        assert_eq!(visitor.types, vec![parse_quote!(Config)]);
    }
}
