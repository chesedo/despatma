use syn::Meta;

use crate::processing::{Dependency, Lifetime};

use super::{ErrorVisitorMut, VisitorMut};

/// Get the lifetime of a dependency from the function attributes
pub struct ExtractLifetime;

impl VisitorMut for ExtractLifetime {
    fn visit_dependency_mut(&mut self, dependency: &mut Dependency) {
        // Remove all lifetime attributes
        dependency.attrs.retain(|attr| {
            let Meta::Path(ref path) = attr.meta else {
                return true;
            };

            if path.segments.len() != 1 {
                return true;
            }

                match path.segments[0].ident.to_string().as_str() {
                    "Transient" => {
                        dependency.lifetime = Lifetime::Transient;
                        false
                    }
                    "Scoped" => {
                        dependency.lifetime = Lifetime::Scoped;
                        false
                    }

                    "Singleton" => {
                        dependency.lifetime = Lifetime::Singleton;
                        false
                    }
                    _ => true,
                }
        });
    }
}

impl ErrorVisitorMut for ExtractLifetime {
    fn new() -> Self {
        Self
    }
    }

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use crate::{
        input,
        processing::{self, visitor::VisitableMut},
    };

    use super::*;

    #[test]
    fn extract_lifetime() {
        let mut container: processing::Container = input::Container::from_item_impl(parse_quote!(
            impl Container {
                #[Singleton]
                fn singleton(&self) -> Singleton {
                    Singleton
                }

                #[Scoped]
                fn scoped(&self) -> Scoped {
                    Scoped
                }

                #[Transient]
                fn transient(&self) -> Transient {
                    Transient
                }

                fn default(&self) -> Default {
                    Default
                }
            }
        ))
        .into();

        assert_eq!(container.dependencies[0].borrow().attrs.len(), 1);
        assert_eq!(
            container.dependencies[0].borrow().lifetime,
            Lifetime::Transient
        );
        assert_eq!(container.dependencies[1].borrow().attrs.len(), 1);
        assert_eq!(
            container.dependencies[1].borrow().lifetime,
            Lifetime::Transient
        );
        assert_eq!(container.dependencies[2].borrow().attrs.len(), 1);
        assert_eq!(
            container.dependencies[2].borrow().lifetime,
            Lifetime::Transient
        );
        assert_eq!(container.dependencies[3].borrow().attrs.len(), 0);
        assert_eq!(
            container.dependencies[3].borrow().lifetime,
            Lifetime::Transient
        );

        container.apply_mut(&mut ExtractLifetime);

        assert_eq!(container.dependencies[0].borrow().attrs.len(), 0);
        assert_eq!(
            container.dependencies[0].borrow().lifetime,
            Lifetime::Singleton
        );
        assert_eq!(container.dependencies[1].borrow().attrs.len(), 0);
        assert_eq!(
            container.dependencies[1].borrow().lifetime,
            Lifetime::Scoped
        );
        assert_eq!(container.dependencies[2].borrow().attrs.len(), 0);
        assert_eq!(
            container.dependencies[2].borrow().lifetime,
            Lifetime::Transient
        );
        assert_eq!(container.dependencies[3].borrow().attrs.len(), 0);
        assert_eq!(
            container.dependencies[3].borrow().lifetime,
            Lifetime::Transient
        );
    }
}
