use syn::parse_quote;

use crate::processing::Dependency;

use super::{ErrorVisitorMut, VisitorMut};

/// Correctly restores the boxes that were removed from any types
///
/// Needs to happen after boxes and lifetimes are extracted
pub struct WrapBoxType;

impl VisitorMut for WrapBoxType {
    fn visit_dependency_mut(&mut self, dependency: &mut Dependency) {
        if dependency.is_boxed {
            let ty = &dependency.ty;

            if dependency.lifetime.is_managed() {
                dependency.field_ty = parse_quote!(std::boxed::Box<#ty + 'a>);
                dependency.ty = parse_quote!(std::boxed::Box<#ty + 'a>);
            } else {
                dependency.field_ty = parse_quote!(std::boxed::Box<#ty>);
                dependency.ty = parse_quote!(std::boxed::Box<#ty>);
            }
        }
    }
}

impl ErrorVisitorMut for WrapBoxType {
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
            visitor::{ExtractBoxType, ExtractLifetime, VisitableMut},
        },
    };

    use super::*;

    #[test]
    fn wrap_box_type() {
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

        assert_eq!(container.dependencies[0].borrow().ty, parse_quote!(dyn DAL));
        assert_eq!(
            container.dependencies[0].borrow().field_ty,
            parse_quote!(Box<dyn DAL>)
        );
        assert_eq!(container.dependencies[1].borrow().ty, parse_quote!(Utc));
        assert_eq!(
            container.dependencies[1].borrow().field_ty,
            parse_quote!(Box<Utc>)
        );
        assert_eq!(
            container.dependencies[2].borrow().ty,
            parse_quote!(Service<impl DAL>),
        );
        assert_eq!(
            container.dependencies[2].borrow().field_ty,
            parse_quote!(Service<impl DAL>),
        );

        container.apply_mut(&mut WrapBoxType);

        assert_eq!(
            container.dependencies[0].borrow().ty,
            parse_quote!(std::boxed::Box<dyn DAL + 'a>),
        );
        assert_eq!(
            container.dependencies[0].borrow().field_ty,
            parse_quote!(std::boxed::Box<dyn DAL + 'a>)
        );
        assert_eq!(
            container.dependencies[1].borrow().ty,
            parse_quote!(std::boxed::Box<Utc>),
        );
        assert_eq!(
            container.dependencies[1].borrow().field_ty,
            parse_quote!(std::boxed::Box<Utc>)
        );
        assert_eq!(
            container.dependencies[2].borrow().ty,
            parse_quote!(Service<impl DAL>),
        );
        assert_eq!(
            container.dependencies[2].borrow().field_ty,
            parse_quote!(Service<impl DAL>)
        );
    }
}
