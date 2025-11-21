use crate::processing::visitor::{ErrorVisitorMut, VisitorMut};
use crate::processing::{Container, Dependency};
use std::cell::RefCell;
use std::rc::Rc;
use syn::{FnArg, Pat, Signature};

/// Creates new dependencies based on constructor arguments
pub struct ExtractEmbeddedDependency;

impl VisitorMut for ExtractEmbeddedDependency {
    fn visit_container_mut(&mut self, container: &mut Container) {
        let mut new_fn: Option<Signature> = None;
        container.dependencies.retain(|dep| {
            let dep_ref = dep.borrow();

            if dep_ref.sig.ident == "new" {
                new_fn = Some(dep_ref.sig.clone());
                false
            } else {
                true
            }
        });

        let Some(new_fn) = new_fn else {
            return;
        };

        container.dependencies = container
            .dependencies
            .iter()
            .cloned()
            .chain(
                new_fn
                    .inputs
                    .iter()
                    .filter_map(|arg| match arg {
                        FnArg::Typed(pat_type) => {
                            let Pat::Ident(_) = pat_type.pat.as_ref() else {
                                return None;
                            };

                            Some(pat_type)
                        }
                        _ => None,
                    })
                    .map(Dependency::from)
                    .map(RefCell::new)
                    .map(Rc::new),
            )
            .collect();
    }
}

impl ErrorVisitorMut for ExtractEmbeddedDependency {
    fn new() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use syn::parse_quote;

    use super::*;
    use crate::{input, processing::visitor::VisitableMut};

    #[test]
    fn extract_input_dependency() {
        let mut container: Container = input::Container::from_item_impl(parse_quote!(
            impl DependencyContainer {
                fn new(config: Config) -> Self {
                    Self
                }

                fn service(&self, config: &Config) -> Service {
                    Service::new(config)
                }
            }
        ))
        .into();

        let mut visitor = ExtractEmbeddedDependency::new();
        container.apply_mut(&mut visitor);

        assert_eq!(container.dependencies.len(), 2);
        let service = container.dependencies[0].clone();
        let config = container.dependencies[1].clone();

        assert_eq!(service.borrow().sig.ident, "service");
        assert_eq!(config.borrow().sig.ident, "config");
        assert_eq!(
            config.borrow().sig,
            parse_quote!(fn config(&self) -> Config)
        );
        assert_eq!(config.borrow().ty, parse_quote!(Config));
        assert_eq!(config.borrow().field_ty, parse_quote!(Config));
    }
}
