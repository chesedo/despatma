use crate::processing::{Dependency, Lifetime};

use super::{ErrorVisitorMut, VisitorMut};

/// Correctly identifies dependencies which needs an explicit lifetime to be added to their outputs and tree.
///
/// Needs to be called after any boxes are extracted.
/// And after lifetimes are extracted.
pub struct SetHasExplicitLifetime;

impl VisitorMut for SetHasExplicitLifetime {
    fn visit_dependency_mut(&mut self, dependency: &mut Dependency) {
        if dependency.is_boxed
            && matches!(dependency.lifetime, Lifetime::Singleton(_) | Lifetime::Scoped(_))
        {
            dependency.has_explicit_lifetime = true;
        }
    }
}

impl ErrorVisitorMut for SetHasExplicitLifetime {
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
            visitor::{ExtractBoxType, ExtractLifetime, VisitableMut},
        },
    };

    use super::*;

    #[test]
    fn set_has_explicit_lifetime() {
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

        assert!(!container.dependencies[0].borrow().has_explicit_lifetime);
        assert!(!container.dependencies[1].borrow().has_explicit_lifetime);
        assert!(!container.dependencies[2].borrow().has_explicit_lifetime);

        container.apply_mut(&mut SetHasExplicitLifetime);

        assert!(container.dependencies[0].borrow().has_explicit_lifetime);
        assert!(!container.dependencies[1].borrow().has_explicit_lifetime);
        assert!(!container.dependencies[2].borrow().has_explicit_lifetime);
    }
}
