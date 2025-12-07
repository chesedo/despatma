use proc_macro_error2::emit_error;
use syn::Type;

use crate::processing::{Dependency, Lifetime};

use super::{ErrorVisitorMut, VisitorMut};

/// Creates errors for any final fields which might be of type impl Trait.
///
/// Needs to be called after extracting lifetimes and embedded dependencies
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
        if let Type::ImplTrait(_) = dependency.field_ty {
            self.errors.push(Error {
                ty: dependency.field_ty.clone(),
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
                Lifetime::Transient(Some(span)) => {
                    emit_error!(
                        ty, "Need to know which type to store for anything which might depend on this transient dependency";
                        hint = span => "Consider adding a type hint to the lifetime attribute";
                        example = "#[Transient(TransientType)]"
                    );
                }
                Lifetime::Transient(None) => {
                    emit_error!(
                        ty, "Need to know which type to store for anything which might depend on this transient dependency";
                        hint = "Add a transient lifetime attribute with a hint type";
                        example = "#[Transient(TransientType)]"
                    );
                }
                Lifetime::Embedded(span) => {
                    emit_error!(
                        ty, "Only concrete types supported";
                        hint = span => "Change this `impl` type to a concrete type";
                    )
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        input,
        processing::{
            self,
            visitor::{ExtractLifetime, VisitableMut},
        },
    };
    use pretty_assertions::assert_eq;
    use proc_macro2::Span;
    use syn::parse_quote;

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

                #[Transient]
                fn transient_impl_trait(&self) -> impl TransientTrait {
                    TransientStruct
                }

                fn default_impl_trait(&self) -> impl DefaultTrait {
                    DefaultStruct
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
                },
                Error {
                    ty: parse_quote!(impl TransientTrait),
                    lifetime: Lifetime::Transient(Some(Span::call_site()))
                },
                Error {
                    ty: parse_quote!(impl DefaultTrait),
                    lifetime: Lifetime::Transient(None)
                },
            ]
        )
    }
}
