use proc_macro_error::emit_error;
use syn::Type;

use crate::processing::{Dependency, Lifetime};

use super::{ErrorVisitorMut, VisitorMut};

/// Creates errors for any final fields which might be of type impl Trait.
///
/// Needs to be called after extracting lifetimes
pub struct ImplTraitFields {
    errors: Vec<Error>,
}

#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
struct Error {
    ty: Type,
    lifetime: Lifetime,
}

impl VisitorMut for ImplTraitFields {
    fn visit_dependency_mut(&mut self, dependency: &mut Dependency) {
        let Some(field_ty) = &dependency.field_ty else {
            return;
        };

        if let Type::ImplTrait(_) = field_ty {
            self.errors.push(Error {
                ty: field_ty.clone(),
                lifetime: dependency.lifetime.clone(),
            });
        }
    }
}

impl ErrorVisitorMut for ImplTraitFields {
    fn new() -> Self {
        Self {
            errors: Default::default(),
        }
    }

    fn emit_errors(self) {
        let Self { errors } = self;

        for Error { ty, lifetime } in errors {
            match lifetime {
                Lifetime::Scoped(span) => {
                    emit_error!(
                        ty, "Need to know which type to store to manage this scoped dependency";
                        hint = span => "Consider adding a type hint to the lifetime attribute";
                        example = "#[Scoped(ScopedType)]"
                    );
                }
                Lifetime::Singleton(span) => {
                    emit_error!(
                        ty, "Need to know which type to store to manage this singleton dependency";
                        hint = span => "Consider adding a type hint to the lifetime attribute";
                        example = "#[Singleton(SingletonType)]"
                    );
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use proc_macro2::Span;
    use syn::parse_quote;

    use crate::{
        input,
        processing::{
            self,
            visitor::{ExtractLifetime, VisitableMut},
        },
    };

    use super::*;

    #[test]
    fn impl_trait_fields() {
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

                #[Singleton]
                fn singleton_impl_trait(&self) -> impl SingletonTrait {
                    SingletonStruct
                }

                #[Scoped]
                fn scoped(&self) -> impl ScopedTrait {
                    ScopedStruct
                }
            }
        ))
        .into();

        container.apply_mut(&mut ExtractLifetime);

        let mut visitor = ImplTraitFields::new();
        container.apply_mut(&mut visitor);

        assert_eq!(
            visitor.errors,
            vec![
                Error {
                    ty: parse_quote!(impl SingletonTrait),
                    lifetime: Lifetime::Singleton(Span::call_site())
                },
                Error {
                    ty: parse_quote!(impl ScopedTrait),
                    lifetime: Lifetime::Scoped(Span::call_site())
                }
            ]
        )
    }
}
