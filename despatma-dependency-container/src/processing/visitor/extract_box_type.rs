use syn::{GenericArgument, PathArguments, Type};

use crate::processing::Dependency;

use super::{ErrorVisitorMut, VisitorMut};

/// Extracts the internal type of any boxed dependency
/// This is needed to fix the Rust lifetimes on any singleton and scoped dependencies
pub struct ExtractBoxType;

impl VisitorMut for ExtractBoxType {
    fn visit_dependency_mut(&mut self, dependency: &mut Dependency) {
        let Type::Path(path) = &dependency.ty else {
            return;
        };

        let Some(last_segment) = path.path.segments.last() else {
            return;
        };

        if last_segment.ident != "Box" {
            return;
        }

        let PathArguments::AngleBracketed(generics) = &last_segment.arguments else {
            return;
        };

        // A box can only have one generic type
        if generics.args.len() != 1 {
            return;
        }

        let GenericArgument::Type(ty) = &generics.args[0] else {
            return;
        };

        dependency.is_boxed = true;
        dependency.ty = ty.clone();
    }
}

impl ErrorVisitorMut for ExtractBoxType {
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
    fn extract_box_type() {
        let mut container: processing::Container = input::Container::from_item_impl(parse_quote!(
            impl Container {
                #[Singleton]
                fn singleton(&self) -> Box<dyn DAL> {
                    Box::new(Postgres)
                }

                fn dal(&self) -> std::boxed::Box<dyn DAL> {
                    Box::new(Sqlite)
                }
            }
        ))
        .into();

        assert!(!container.dependencies[0].borrow().is_boxed);
        assert_eq!(
            container.dependencies[0].borrow().ty,
            parse_quote!(Box<dyn DAL>),
        );
        assert!(!container.dependencies[1].borrow().is_boxed);
        assert_eq!(
            container.dependencies[1].borrow().ty,
            parse_quote!(std::boxed::Box<dyn DAL>),
        );

        container.apply_mut(&mut ExtractBoxType);

        assert!(container.dependencies[0].borrow().is_boxed);
        assert_eq!(container.dependencies[0].borrow().ty, parse_quote!(dyn DAL));
        assert!(container.dependencies[1].borrow().is_boxed);
        assert_eq!(container.dependencies[1].borrow().ty, parse_quote!(dyn DAL));
    }
}
