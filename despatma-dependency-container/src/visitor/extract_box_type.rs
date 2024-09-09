use syn::{GenericArgument, PathArguments, Type};

use crate::container::Dependency;

use super::{visit_dependency_mut, VisitorMut};

/// Extracts the internal type of any boxed dependency
/// This is needed to fix the Rust lifetimes on any singleton and scoped dependencies
pub struct ExtractBoxType;

impl ExtractBoxType {
    pub fn new() -> Self {
        Self
    }
}

impl VisitorMut for ExtractBoxType {
    fn visit_dependency_mut(&mut self, dependency: &mut Dependency) {
        visit_dependency_mut(self, dependency);

        let Type::Path(ref path) = dependency.ty else {
            return;
        };

        let Some(last_segment) = path.path.segments.last() else {
            return;
        };

        if last_segment.ident != "Box" {
            return;
        }

        let PathArguments::AngleBracketed(generics): &PathArguments = &last_segment.arguments
        else {
            return;
        };

        // A box can only have one have one generic type
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

#[cfg(test)]
mod tests {
    use syn::{parse_quote, Ident};

    use crate::{container::Container, visitor::VisitableMut};

    use super::*;

    #[test]
    fn extract_box_type() {
        let mut container = Container::from_item_impl(parse_quote!(
            impl DependencyContainer {
                #[Singleton]
                fn singleton(&self) -> Box<dyn DAL> {
                    Box::new(Postgres)
                }

                fn dal(&self) -> std::boxed::Box<dyn DAL> {
                    Box::new(Sqlite)
                }
            }
        ));

        let mut extract_box_visitor = ExtractBoxType::new();
        container.apply_mut(&mut extract_box_visitor);

        assert!(
            container
                .dependencies
                .get::<Ident>(&parse_quote!(singleton))
                .unwrap()
                .borrow()
                .is_boxed
        );
        assert_eq!(
            container
                .dependencies
                .get::<Ident>(&parse_quote!(singleton))
                .unwrap()
                .borrow()
                .ty,
            parse_quote!(dyn DAL),
        );

        assert!(
            container
                .dependencies
                .get::<Ident>(&parse_quote!(dal))
                .unwrap()
                .borrow()
                .is_boxed
        );
        assert_eq!(
            container
                .dependencies
                .get::<Ident>(&parse_quote!(dal))
                .unwrap()
                .borrow()
                .ty,
            parse_quote!(dyn DAL),
        );
    }
}
