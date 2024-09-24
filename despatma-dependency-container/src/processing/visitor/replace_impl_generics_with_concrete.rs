use std::collections::HashMap;

use syn::{
    parse_quote,
    visit_mut::{visit_type_mut, VisitMut},
    Type,
};

use crate::processing::Dependency;

use super::{ErrorVisitorMut, VisitorMut};

/// Replaces any `impl Trait` in generics with their explicitly hinted concrete types.
///
/// Needs to happen after type hints (lifetimes) are extracted.
/// And after dependencies are linked.
pub struct ReplaceImplGenericsWithConcrete {
    replacer: Replacer,
}

impl VisitorMut for ReplaceImplGenericsWithConcrete {
    fn visit_dependency_mut(&mut self, dependency: &mut Dependency) {
        if let Some(field_ty) = &mut dependency.field_ty {
            if self.replacer.to_replace.contains_key(&dependency.ty) {
                return;
            }

            // Replace children first as they might impact this parent type
            for dependency in dependency.dependencies.iter_mut() {
                self.visit_dependency_mut(&mut dependency.inner.borrow_mut());
            }

            // Fix `field_ty` first before it is registered for later replacements
            self.replacer.visit_type_mut(field_ty);

            self.replacer
                .to_replace
                .insert(dependency.ty.clone(), field_ty.clone());
        }
    }
}

struct Replacer {
    to_replace: HashMap<Type, Type>,
}

impl VisitMut for Replacer {
    fn visit_type_mut(&mut self, ty: &mut Type) {
        if let Some(concrete_type) = self.to_replace.get(ty) {
            *ty = parse_quote! { &'a #concrete_type };
        } else {
            // Continue checking for any impl types on inner generics
            visit_type_mut(self, ty);
        }
    }
}

impl ErrorVisitorMut for ReplaceImplGenericsWithConcrete {
    fn new() -> Self {
        Self {
            replacer: Replacer {
                to_replace: Default::default(),
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
            visitor::{ExtractLifetime, LinkDependencies, VisitableMut},
        },
    };

    use super::*;

    #[test]
    fn impl_trait_but_registered_concrete() {
        let mut container: processing::Container = input::Container::from_item_impl(parse_quote!(
            impl Container {
                #[Singleton(Sqlite)]
                fn db(&self) -> impl DB {
                    Sqlite
                }

                #[Singleton]
                fn service(&self, db: impl DB) -> Service<impl DB> {
                    Service(db)
                }
            }
        ))
        .into();

        // Test needs them to be linked and lifetimes to be extracted
        container.apply_mut(&mut ExtractLifetime::new());
        container.apply_mut(&mut LinkDependencies::new());

        assert_eq!(
            container.dependencies[0].borrow().field_ty,
            Some(parse_quote!(Sqlite))
        );
        assert_eq!(
            container.dependencies[1].borrow().field_ty,
            Some(parse_quote!(Service<impl DB>))
        );

        container.apply_mut(&mut ReplaceImplGenericsWithConcrete::new());

        assert_eq!(
            container.dependencies[0].borrow().field_ty,
            Some(parse_quote!(Sqlite))
        );
        assert_eq!(
            container.dependencies[1].borrow().field_ty,
            Some(parse_quote!(Service<&'a Sqlite>))
        );
    }
}
