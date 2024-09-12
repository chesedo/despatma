use proc_macro_error::emit_error;
use quote::ToTokens;
use syn::{FnArg, Pat, PatType, Type, TypeImplTrait};

use crate::processing::Dependency;

use super::{ErrorVisitorMut, VisitorMut};

/// Visitor to find any requested dependencies of a concrete type, while the registered dependency
/// returns an `impl Trait`.
///
/// Needs to happen after child dependencies have been linked.
/// But before any types are changed.
pub struct ImplTraitButRegisteredConcrete {
    errors: Vec<Error>,
}

#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
struct Error {
    requested: PatType,
    registered: TypeImplTrait,
}

impl VisitorMut for ImplTraitButRegisteredConcrete {
    fn visit_dependency_mut(&mut self, dependency: &mut Dependency) {
        let path_inputs = dependency
            .sig
            .inputs
            .iter()
            .filter_map(|input| match input {
                FnArg::Receiver(_) => None,
                FnArg::Typed(pat_type) => match pat_type.ty.as_ref() {
                    Type::Path(_) => Some(pat_type),
                    _ => None,
                },
            });

        for pat_type in path_inputs {
            let child_ident = match pat_type.pat.as_ref() {
                Pat::Ident(pat_ident) => pat_ident.ident.clone(),
                _ => continue,
            };

            let Some(child_dependency) = dependency
                .dependencies
                .iter()
                .map(|d| &d.inner)
                .find(|d| d.borrow().sig.ident == child_ident)
            else {
                continue;
            };

            let child_dependency = child_dependency.borrow();
            let child_return_type = match &child_dependency.ty {
                Type::ImplTrait(impl_trait) => impl_trait,
                _ => continue,
            };

            self.errors.push(Error {
                requested: pat_type.clone(),
                registered: child_return_type.clone(),
            });
        }
    }
}

impl ErrorVisitorMut for ImplTraitButRegisteredConcrete {
    fn new() -> Self {
        Self {
            errors: Default::default(),
        }
    }

    fn emit_errors(self) {
        let errors = self.errors;

        for Error {
            requested,
            registered,
        } in errors
        {
            emit_error!(
                requested,
                "Requested type is a concrete type, but the registered type is: `{}`",
                registered.to_token_stream();
                hint = "change this to `{}: {}`", requested.pat.to_token_stream(), registered.to_token_stream()
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
            visitor::{LinkDependencies, VisitableMut},
        },
    };

    use super::*;

    #[test]
    fn impl_trait_but_registered_concrete() {
        let mut container: processing::Container = input::Container::from_item_impl(parse_quote!(
            impl Container {
                fn db(&self) -> impl DB {
                    Sqlite
                }

                fn service(&self, db: Sqlite) -> Service {
                    Service(db)
                }
            }
        ))
        .into();

        // Test needs them to be linked
        container.apply_mut(&mut LinkDependencies::new());

        let mut visitor = ImplTraitButRegisteredConcrete::new();

        container.apply_mut(&mut visitor);

        assert_eq!(
            visitor.errors,
            vec![Error {
                requested: parse_quote!(db: Sqlite),
                registered: parse_quote!(impl DB),
            }]
        );
    }
}
