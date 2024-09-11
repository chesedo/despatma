use crate::processing::Container;

use super::{ErrorVisitorMut, VisitorMut};

/// Checks whether the container needs a generic lifetime.
///
/// Needs to be called after explicit lifetimes are set.
pub struct SetNeedsGenericLifetime;

impl VisitorMut for SetNeedsGenericLifetime {
    fn visit_container_mut(&mut self, container: &mut Container) {
        container.needs_generic_lifetime = container.dependencies.iter().any(|d| d.borrow().has_explicit_lifetime);
    }
}

impl ErrorVisitorMut for SetNeedsGenericLifetime {
    fn new() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use crate::{
        input,
        processing::{
            self,
            visitor::{
                ExtractBoxType, ExtractLifetime, SetHasExplicitLifetime,
                VisitableMut,
            },
        },
    };

    use super::*;

    #[test]
    fn set_needs_generic_lifetime() {
        let mut container: processing::Container = input::Container::from_item_impl(parse_quote!(
            impl Container {
                #[Singleton]
                fn dal(&self) -> Box<dyn DAL> {
                    Box::new(Postgres)
                }

                fn datetime(&self) -> Box<Utc> {
                    Box::new(Utc::now())
                }

                fn service(&self, dal: impl DAL, datetime: Utc) -> Service<impl DAL> {
                    Service::new(dal, datetime)
                }
            }
        ))
        .into();

        container.apply_mut(&mut ExtractBoxType);
        container.apply_mut(&mut ExtractLifetime);
        container.apply_mut(&mut SetHasExplicitLifetime);

        assert!(!container.needs_generic_lifetime);

        container.apply_mut(&mut SetNeedsGenericLifetime);

        assert!(container.needs_generic_lifetime);
    }
}
