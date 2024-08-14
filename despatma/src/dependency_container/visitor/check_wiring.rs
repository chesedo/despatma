use proc_macro_error::emit_error;
use strsim::levenshtein;
use syn::Ident;

use crate::dependency_container::ChildDependency;

use super::{visit_child_dependency, Visit};

/// This visitor is responsible for checking if all dependencies have been registered in the container.
/// So if `a` has a dependency on `b`, this visitor will check if `b` has been registered in the container.
/// If not, it will emit an error.
pub struct CheckWiring {
    dependencies: Vec<Ident>,
    errors: Vec<Error>,
}

#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
struct Error {
    requested: Ident,
    best_match: Option<Ident>,
}

impl CheckWiring {
    /// Create a new `WiringVisitor` with the given available dependencies.
    /// Ie. if this visitor finds a requested child dependency which is not in the list of available dependencies,
    /// it will emit an error.
    pub fn new(dependencies: Vec<Ident>) -> Self {
        Self {
            dependencies,
            errors: Vec::new(),
        }
    }
}

impl Visit for CheckWiring {
    fn visit_child_dependency(&mut self, child_dependency: &ChildDependency) {
        if !self.dependencies.contains(&child_dependency.ident) {
            let best_match =
                get_best_dependency_match(&self.dependencies, &child_dependency.ident.to_string());

            self.errors.push(Error {
                requested: child_dependency.ident.clone(),
                best_match,
            });
        }

        // Keep traversing the tree
        visit_child_dependency(self, child_dependency);
    }

    fn emit_errors(self) {
        let Self { errors, .. } = self;

        for error in errors {
            if let Some(best_match) = error.best_match {
                emit_error!(error.requested, "The '{}' dependency has not been registered", error.requested; hint = best_match.span() => format!("Did you mean `{}`?", best_match));
            } else {
                emit_error!(
                    error.requested,
                    "Dependency not found. Did you forget to add it?";
                    hint = "Try adding it with `fn {}(&self) ...`", error.requested
                );
            }
        }
    }
}

/// The maximum distance between two strings for them to be considered a misspelling.
const MISSPELLING_THRESHOLD: usize = 3;

fn get_best_dependency_match(dependencies: &[Ident], needle: &str) -> Option<Ident> {
    dependencies
        .iter()
        .map(|d| (d, levenshtein(needle, &d.to_string())))
        .filter(|(_, distance)| *distance <= MISSPELLING_THRESHOLD)
        .min_by_key(|(_, distance)| *distance)
        .map(|(d, _)| d.clone())
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use crate::dependency_container::Container;

    use super::*;

    #[test]
    fn check_wiring() {
        let mut wiring_visitor = CheckWiring::new(vec![
            Ident::new("foo", proc_macro2::Span::call_site()),
            Ident::new("bar", proc_macro2::Span::call_site()),
            Ident::new("baz", proc_macro2::Span::call_site()),
        ]);
        let container = Container::from_item_impl(parse_quote!(
            impl Dependencies {
                fn service(&self, zoo: Foo, barrier: Barrier, baz: Baz) -> Service {
                    Service
                }
            }
        ));

        wiring_visitor.visit_container(&container);

        let CheckWiring { errors, .. } = wiring_visitor;

        assert_eq!(
            errors,
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
