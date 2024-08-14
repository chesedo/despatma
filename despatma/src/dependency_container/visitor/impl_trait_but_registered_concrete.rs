use std::{cell::RefCell, rc::Rc};

use indexmap::IndexMap;
use proc_macro_error::emit_error;
use quote::ToTokens;
use syn::{Ident, PatType, TypeImplTrait};

use crate::dependency_container::Dependency;

use super::Visit;

/// Visitor to find any requested dependencies of a concrete type, while the registered dependency
/// returns an `impl Trait`.
pub struct ImplTraitButRegisteredConcrete {
    dependencies: IndexMap<Ident, Rc<RefCell<Dependency>>>,
    errors: Vec<Error>,
}

#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
struct Error {
    requested: PatType,
    registered: TypeImplTrait,
}

impl ImplTraitButRegisteredConcrete {
    pub fn new(dependencies: IndexMap<Ident, Rc<RefCell<Dependency>>>) -> Self {
        Self {
            dependencies,
            errors: Vec::new(),
        }
    }
}

impl Visit for ImplTraitButRegisteredConcrete {
    fn visit_dependency(&mut self, dependency: &Dependency) {
        for pat_type in dependency
            .sig
            .inputs
            .iter()
            .filter_map(|input| match input {
                syn::FnArg::Receiver(_) => None,
                syn::FnArg::Typed(pat_type) => match pat_type.ty.as_ref() {
                    // syn::Type::ImplTrait(impl_trait) => Some(pat_type),
                    syn::Type::Path(_) => Some(pat_type),
                    _ => None,
                },
            })
        {
            let child_ident = match pat_type.pat.as_ref() {
                syn::Pat::Ident(pat_ident) => pat_ident.ident.clone(),
                _ => continue,
            };

            if let Some(child_dependency) = self.dependencies.get(&child_ident) {
                let child_dependency = child_dependency.borrow();
                let child_return_type = match &child_dependency.sig.output {
                    // Handled by another validator
                    syn::ReturnType::Default => continue,
                    syn::ReturnType::Type(_, ty) => match ty.as_ref() {
                        syn::Type::ImplTrait(impl_trait) => impl_trait,
                        _ => continue,
                    },
                };

                self.errors.push(Error {
                    requested: pat_type.clone(),
                    registered: child_return_type.clone(),
                });
            }
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
    use syn::parse_quote;

    use crate::dependency_container::Container;

    use super::*;

    #[test]
    fn invalid_case() {
        let container = Container::from_item_impl(parse_quote!(
            impl Dependencies {
                fn db(&self) -> impl DB {
                    Sqlite
                }

                fn service(&self, db: Sqlite) -> Service {
                    Service(db)
                }
            }
        ));

        let mut visitor = ImplTraitButRegisteredConcrete::new(container.dependencies.clone());
        visitor.visit_container(&container);

        let ImplTraitButRegisteredConcrete { errors, .. } = visitor;

        assert_eq!(
            errors,
            vec![Error {
                requested: parse_quote!(db: Sqlite),
                registered: parse_quote!(impl DB),
            }]
        );
    }
}
