use std::{cell::RefCell, collections::HashMap, rc::Rc};

use proc_macro_error2::emit_error;
use strsim::levenshtein;
use syn::{FnArg, Ident, Pat};

use crate::processing::{ChildDependency, Container, Dependency};

use super::{visit_container_mut, ErrorVisitorMut, VisitorMut};

/// Extracts any child dependencies correctly from the registered dependencies and report on any requested dependencies that are not registered.
/// So if `a` has a dependency on `b`, this visitor will check if `b` has been registered in the container.
/// If not, it will emit an error.
pub struct LinkDependencies {
    dependencies: HashMap<Ident, Rc<RefCell<Dependency>>>,
    errors: Vec<Error>,
}

#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
struct Error {
    requested: Ident,
    best_match: Option<Ident>,
}

impl VisitorMut for LinkDependencies {
    fn visit_container_mut(&mut self, container: &mut Container) {
        self.dependencies = HashMap::from_iter(container.dependencies.iter().map(|d| {
            let d_ref = d.borrow();
            (d_ref.sig.ident.clone(), d.clone())
        }));

        visit_container_mut(self, container)
    }

    fn visit_dependency_mut(&mut self, dependency: &mut Dependency) {
        let dependencies = dependency
            .sig
            .inputs
            .iter()
            .filter_map(|fn_arg| {
                let FnArg::Typed(pat_type) = fn_arg else {
                    return None;
                };

                let Pat::Ident(pat) = pat_type.pat.as_ref() else {
                    return None;
                };

                let Some(child_dependency) =
                    self.dependencies.get(&pat.ident).map(|d| ChildDependency {
                        inner: d.clone(),
                        ty: pat_type.ty.as_ref().clone(),
                    })
                else {
                    let best_match = get_best_dependency_match(
                        &self.dependencies.keys().collect::<Vec<_>>(),
                        &pat.ident.to_string(),
                    );

                    self.errors.push(Error {
                        requested: pat.ident.clone(),
                        best_match,
                    });

                    return None;
                };

                Some(child_dependency)
            })
            .collect();

        dependency.dependencies = dependencies;
    }
}

impl ErrorVisitorMut for LinkDependencies {
    fn new() -> Self {
        Self {
            dependencies: Default::default(),
            errors: Default::default(),
        }
    }

    fn emit_errors(self) {
        let Self { errors, .. } = self;

        for Error {
            requested,
            best_match,
        } in errors
        {
            if let Some(best_match) = best_match {
                emit_error!(
                    requested,
                    "The '{}' dependency has not been registered",
                    requested;
                    hint = best_match.span() => format!("Did you mean `{}`?", best_match)
                );
            } else {
                emit_error!(
                    requested,
                    "Dependency not found. Did you forget to add it?";
                    hint = "Try adding it with `fn {}(&self) ...`", requested
                );
            }
        }
    }
}

/// The maximum distance between two strings for them to be considered a misspelling.
const MISSPELLING_THRESHOLD: usize = 3;

fn get_best_dependency_match(dependencies: &[&Ident], needle: &str) -> Option<Ident> {
    dependencies
        .iter()
        .map(|d| (d, levenshtein(needle, &d.to_string())))
        .filter(|(_, distance)| *distance <= MISSPELLING_THRESHOLD)
        .min_by_key(|(_, distance)| *distance)
        .map(|(d, _)| *d)
        .cloned()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use syn::parse_quote;

    use crate::{
        input,
        processing::{self, visitor::VisitableMut},
    };

    use super::*;

    #[test]
    fn link_dependencies() {
        let mut container: processing::Container = input::Container::from_item_impl(parse_quote!(
            impl Container {
                fn config(&self) -> Config {
                    Config
                }

                fn service(&self, config: &Config, zoo: Foo, barrier: Barrier, baz: Baz) -> Service {
                    Service::new(config)
                }

                fn foo(&self) -> Foo {
                    Foo
                }

                fn bar(&self) -> Bar {
                    Bar
                }

                fn baz(&self) -> Baz {
                    Baz
                }
            }
        ))
        .into();

        assert!(container.dependencies[0].borrow().dependencies.is_empty());
        assert!(container.dependencies[1].borrow().dependencies.is_empty());

        let mut visitor = LinkDependencies::new();

        container.apply_mut(&mut visitor);

        assert!(container.dependencies[0].borrow().dependencies.is_empty());
        assert_eq!(
            container.dependencies[1].borrow().dependencies[0]
                .inner
                .borrow()
                .sig
                .ident,
            "config"
        );
        assert_eq!(
            container.dependencies[1].borrow().dependencies[0].ty,
            parse_quote!(&Config)
        );

        assert_eq!(
            visitor.errors,
            vec![
                Error {
                    requested: Ident::new("zoo", proc_macro2::Span::call_site()),
                    best_match: Some(Ident::new("foo", proc_macro2::Span::call_site())),
                },
                Error {
                    requested: Ident::new("barrier", proc_macro2::Span::call_site()),
                    best_match: None,
                }
            ]
        );
    }
}
