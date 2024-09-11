use syn::{parse_quote, Meta};

use crate::processing::{Dependency, Lifetime};

use super::{ErrorVisitorMut, VisitorMut};

/// Get the lifetime of a dependency from the function attributes
pub struct ExtractLifetime;

impl VisitorMut for ExtractLifetime {
    fn visit_dependency_mut(&mut self, dependency: &mut Dependency) {
        // Remove all lifetime attributes
        dependency.attrs.retain(|attr| {
            let path = match &attr.meta {
                Meta::Path(path) => path,
                Meta::List(meta_list) => {
                    let custom_type = &meta_list.tokens;
                    let custom_type = parse_quote!(#custom_type);

                    dependency.create_ty = custom_type;

                    &meta_list.path
                }
                Meta::NameValue(_) => return true,
            };

            if path.segments.len() != 1 {
                return true;
            }

            match path.segments[0].ident.to_string().as_str() {
                "Scoped" => {
                    dependency.lifetime = Lifetime::Scoped;
                    dependency.field_ty = Some(dependency.create_ty.clone());
                    false
                }

                "Singleton" => {
                    dependency.lifetime = Lifetime::Singleton;
                    dependency.field_ty = Some(dependency.create_ty.clone());
                    false
                }
                "Transient" => false,
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
    use pretty_assertions::assert_eq;
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

                #[Singleton(SingletonStruct)]
                fn singleton_impl_trait(&self) -> impl SingletonTrait {
                    SingletonStruct
                }

                #[Scoped(ScopedStruct)]
                fn scoped(&self) -> impl ScopedTrait {
                    ScopedStruct
                }
            }
        ))
        .into();

        assert_eq!(container.dependencies[0].borrow().attrs.len(), 1);
        assert_eq!(
            container.dependencies[0].borrow().lifetime,
            Lifetime::Transient
        );
        assert_eq!(
            container.dependencies[0].borrow().create_ty,
            parse_quote!(Singleton)
        );
        assert_eq!(container.dependencies[0].borrow().field_ty, None);
        assert_eq!(container.dependencies[1].borrow().attrs.len(), 1);
        assert_eq!(
            container.dependencies[1].borrow().lifetime,
            Lifetime::Transient
        );
        assert_eq!(
            container.dependencies[1].borrow().create_ty,
            parse_quote!(Scoped)
        );
        assert_eq!(container.dependencies[1].borrow().field_ty, None);
        assert_eq!(container.dependencies[2].borrow().attrs.len(), 1);
        assert_eq!(
            container.dependencies[2].borrow().lifetime,
            Lifetime::Transient
        );
        assert_eq!(
            container.dependencies[2].borrow().create_ty,
            parse_quote!(Transient)
        );
        assert_eq!(container.dependencies[2].borrow().field_ty, None);
        assert_eq!(container.dependencies[3].borrow().attrs.len(), 0);
        assert_eq!(
            container.dependencies[3].borrow().lifetime,
            Lifetime::Transient
        );
        assert_eq!(
            container.dependencies[3].borrow().create_ty,
            parse_quote!(Default)
        );
        assert_eq!(container.dependencies[3].borrow().field_ty, None);
        assert_eq!(container.dependencies[4].borrow().attrs.len(), 1);
        assert_eq!(
            container.dependencies[4].borrow().lifetime,
            Lifetime::Transient
        );
        assert_eq!(
            container.dependencies[4].borrow().create_ty,
            parse_quote!(impl SingletonTrait)
        );
        assert_eq!(container.dependencies[4].borrow().field_ty, None);
        assert_eq!(container.dependencies[5].borrow().attrs.len(), 1);
        assert_eq!(
            container.dependencies[5].borrow().lifetime,
            Lifetime::Transient
        );
        assert_eq!(
            container.dependencies[5].borrow().create_ty,
            parse_quote!(impl ScopedTrait)
        );
        assert_eq!(container.dependencies[5].borrow().field_ty, None);

        container.apply_mut(&mut ExtractLifetime);

        assert_eq!(container.dependencies[0].borrow().attrs.len(), 0);
        assert_eq!(
            container.dependencies[0].borrow().lifetime,
            Lifetime::Singleton
        );
        assert_eq!(
            container.dependencies[0].borrow().create_ty,
            parse_quote!(Singleton)
        );
        assert_eq!(
            container.dependencies[0].borrow().field_ty,
            Some(parse_quote!(Singleton))
        );
        assert_eq!(container.dependencies[1].borrow().attrs.len(), 0);
        assert_eq!(
            container.dependencies[1].borrow().lifetime,
            Lifetime::Scoped
        );
        assert_eq!(
            container.dependencies[1].borrow().create_ty,
            parse_quote!(Scoped)
        );
        assert_eq!(
            container.dependencies[1].borrow().field_ty,
            Some(parse_quote!(Scoped))
        );
        assert_eq!(container.dependencies[2].borrow().attrs.len(), 0);
        assert_eq!(
            container.dependencies[2].borrow().lifetime,
            Lifetime::Transient
        );
        assert_eq!(
            container.dependencies[2].borrow().create_ty,
            parse_quote!(Transient)
        );
        assert_eq!(container.dependencies[2].borrow().field_ty, None);
        assert_eq!(container.dependencies[3].borrow().attrs.len(), 0);
        assert_eq!(
            container.dependencies[3].borrow().lifetime,
            Lifetime::Transient
        );
        assert_eq!(
            container.dependencies[3].borrow().create_ty,
            parse_quote!(Default)
        );
        assert_eq!(container.dependencies[3].borrow().field_ty, None);
        assert_eq!(container.dependencies[4].borrow().attrs.len(), 0);
        assert_eq!(
            container.dependencies[4].borrow().lifetime,
            Lifetime::Singleton
        );
        assert_eq!(
            container.dependencies[4].borrow().create_ty,
            parse_quote!(SingletonStruct)
        );
        assert_eq!(
            container.dependencies[4].borrow().field_ty,
            Some(parse_quote!(SingletonStruct))
        );
        assert_eq!(container.dependencies[5].borrow().attrs.len(), 0);
        assert_eq!(
            container.dependencies[5].borrow().lifetime,
            Lifetime::Scoped
        );
        assert_eq!(
            container.dependencies[5].borrow().create_ty,
            parse_quote!(ScopedStruct)
        );
        assert_eq!(
            container.dependencies[5].borrow().field_ty,
            Some(parse_quote!(ScopedStruct))
        );
    }
}
