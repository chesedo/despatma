use std::collections::HashSet;

use syn::{
    parse_quote,
    punctuated::Punctuated,
    visit_mut::{visit_type_impl_trait_mut, VisitMut},
    Token, Type, TypeImplTrait, TypeParamBound,
};

use crate::processing::Dependency;

use super::{ErrorVisitorMut, VisitorMut};

/// Add the wildcard lifetime to any return types that might need it.
/// This is for dependencies which requests and returns an impl Trait dependency which has a managed lifetime.
///
/// Needs to be called after lifetimes are extracted.
/// And after dependencies are linked.
/// Needs to be called before boxes are wrapped again.
pub struct AddWildcardLifetime {
    adder: Adder,
}

impl VisitorMut for AddWildcardLifetime {
    fn visit_dependency_mut(&mut self, dependency: &mut Dependency) {
        if dependency.lifetime.is_managed() {
            if let Type::ImplTrait(type_impl_trait) = &dependency.ty {
                self.adder.to_add.insert(type_impl_trait.bounds.clone());
                return;
            }
            // Also fix up `dyn Trait` that was exctracted from boxes
            if let Type::TraitObject(type_trait_object) = &dependency.ty {
                self.adder.to_add.insert(type_trait_object.bounds.clone());
                return;
            }
        }

        // Replace children first as they might have an impact on this type
        for dependency in dependency.dependencies.iter_mut() {
            self.visit_dependency_mut(&mut dependency.inner.borrow_mut());
        }

        self.adder.visit_type_mut(&mut dependency.ty);
    }
}

struct Adder {
    to_add: HashSet<Punctuated<TypeParamBound, Token![+]>>,
}

impl VisitMut for Adder {
    fn visit_type_impl_trait_mut(&mut self, type_impl_trait: &mut TypeImplTrait) {
        if self.to_add.contains(&type_impl_trait.bounds) {
            type_impl_trait.bounds.push(parse_quote!('a));
        } else {
            // Continue checking for any impl types on inner generics
            visit_type_impl_trait_mut(self, type_impl_trait);
        }
    }
}

impl ErrorVisitorMut for AddWildcardLifetime {
    fn new() -> Self {
        Self {
            adder: Adder {
                to_add: Default::default(),
            },
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
            visitor::{ExtractBoxType, ExtractLifetime, LinkDependencies, VisitableMut},
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

                #[Scoped]
                fn config(&self) -> impl Config {
                    ConfigStruct::parse()
                }

                fn datetime(&self) -> Box<Utc> {
                    Box::new(Utc::now())
                }

                // Needed to check nested types are updated correctly in `service` below
                #[Scope]
                fn presenter(&self, config: impl Config) -> Presenter<impl Config> {
                    Presenter::new(config)
                }

                fn service(&self, dal: impl DAL, config: impl Config, datetime: Utc, presenter: Presenter<impl Config>) -> Service<impl DAL, impl Config, Presenter<impl Config>> {
                    Service::new(dal, config, datetime, presenter)
                }
            }
        ))
        .into();

        container.apply_mut(&mut ExtractBoxType);
        container.apply_mut(&mut ExtractLifetime);
        container.apply_mut(&mut LinkDependencies::new());

        assert_eq!(container.dependencies[0].borrow().ty, parse_quote!(dyn DAL));
        assert_eq!(
            container.dependencies[1].borrow().ty,
            parse_quote!(impl Config)
        );
        assert_eq!(container.dependencies[2].borrow().ty, parse_quote!(Utc));
        assert_eq!(
            container.dependencies[3].borrow().ty,
            parse_quote!(Presenter<impl Config>),
        );
        assert_eq!(
            container.dependencies[4].borrow().ty,
            parse_quote!(Service<impl DAL, impl Config, Presenter<impl Config>>),
        );

        container.apply_mut(&mut AddWildcardLifetime::new());

        assert_eq!(container.dependencies[0].borrow().ty, parse_quote!(dyn DAL));
        assert_eq!(
            container.dependencies[1].borrow().ty,
            parse_quote!(impl Config)
        );
        assert_eq!(container.dependencies[2].borrow().ty, parse_quote!(Utc));
        assert_eq!(
            container.dependencies[3].borrow().ty,
            parse_quote!(Presenter<impl Config + 'a>),
        );
        assert_eq!(
            container.dependencies[4].borrow().ty,
            parse_quote!(Service<impl DAL + 'a, impl Config + 'a, Presenter<impl Config + 'a>>),
        );
    }
}
