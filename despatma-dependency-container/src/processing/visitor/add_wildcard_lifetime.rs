use syn::{parse_quote, GenericArgument, PathArguments, Type, TypeTraitObject};

use crate::processing::Dependency;

use super::{ErrorVisitorMut, VisitorMut};

/// Add the wildcard lifetime to any return types that might need it.
/// This is for dependencies which requests and returns an impl Trait dependency which has an explicit lifetime.
///
/// Needs to be called after explicit lifetimes are set.
/// Needs to be called before boxes are wrapped again.
pub struct AddWildcardLifetime;

impl VisitorMut for AddWildcardLifetime {
    fn visit_dependency_mut(&mut self, dependency: &mut Dependency) {
        let Type::Path(path) = &mut dependency.ty else {
            return;
        };

        let Some(last_segment) = path.path.segments.last_mut() else {
            return;
        };

        let PathArguments::AngleBracketed(generics) = &mut last_segment.arguments else {
            return;
        };

        let deps_needing_wildcard_lifetime: Vec<_> = dependency
            .dependencies
            .iter()
            .filter(|dep| dep.inner.borrow().has_explicit_lifetime)
            .map(|dep| dep.inner.borrow().ty.clone())
            .filter_map(|ty| match ty {
                Type::TraitObject(TypeTraitObject { bounds, .. }) => Some(bounds),
                _ => None,
            })
            .collect();

        for arg in generics.args.iter_mut() {
            let GenericArgument::Type(Type::ImplTrait(type_impl_trait)) = arg else {
                continue;
            };

            if deps_needing_wildcard_lifetime.contains(&type_impl_trait.bounds) {
                type_impl_trait.bounds.push(parse_quote!('_));
            }
        }
    }
}

impl ErrorVisitorMut for AddWildcardLifetime {
    fn new() -> Self {
        Self
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
            visitor::{
                ExtractBoxType, ExtractLifetime, LinkDependencies, SetHasExplicitLifetime,
                VisitableMut,
            },
        },
    };

    use super::*;

    #[test]
    fn add_wildcard_lifetime() {
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
        container.apply_mut(&mut LinkDependencies::new());

        assert!(container.dependencies[0].borrow().has_explicit_lifetime);
        assert_eq!(container.dependencies[0].borrow().ty, parse_quote!(dyn DAL));
        assert!(!container.dependencies[1].borrow().has_explicit_lifetime);
        assert_eq!(container.dependencies[1].borrow().ty, parse_quote!(Utc));
        assert!(!container.dependencies[2].borrow().has_explicit_lifetime);
        assert_eq!(
            container.dependencies[2].borrow().ty,
            parse_quote!(Service<impl DAL>),
        );

        container.apply_mut(&mut AddWildcardLifetime);

        assert!(container.dependencies[0].borrow().has_explicit_lifetime);
        assert_eq!(container.dependencies[0].borrow().ty, parse_quote!(dyn DAL));
        assert!(!container.dependencies[1].borrow().has_explicit_lifetime);
        assert_eq!(container.dependencies[1].borrow().ty, parse_quote!(Utc));
        assert!(!container.dependencies[2].borrow().has_explicit_lifetime);
        assert_eq!(
            container.dependencies[2].borrow().ty,
            parse_quote!(Service<impl DAL + '_>),
        );
    }
}
