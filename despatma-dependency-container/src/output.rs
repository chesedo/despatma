use std::{cell::RefCell, rc::Rc};

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_quote, parse_str,
    punctuated::Punctuated,
    token::{Async, Await, Fn, Paren},
    Attribute, Block, Field, FieldValue, FieldsNamed, FnArg, Ident, Path, Signature, Token, Type,
};

use crate::processing::{self, Lifetime};

#[cfg(any(test, feature = "standalone"))]
const ASYNC_ONCE_CELL_PATH: &str = "async_once_cell::OnceCell";

#[cfg(not(any(test, feature = "standalone")))]
const ASYNC_ONCE_CELL_PATH: &str = "despatma::async_once_cell::OnceCell";

#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
pub struct Container {
    attrs: Vec<Attribute>,
    self_ty: Type,
    fields: Punctuated<Field, Token![,]>,
    constructors: Punctuated<FieldValue, Token![,]>,
    scope_constructors: Punctuated<FieldValue, Token![,]>,
    dependencies: Vec<Dependency>,
}

#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
pub struct Dependency {
    attrs: Vec<Attribute>,
    block: Block,
    asyncness: Option<Async>,
    fn_token: Fn,
    ident: Ident,
    paren_token: Paren,
    inputs: Punctuated<FnArg, Token![,]>,
    ty: Type,
    create_asyncness: Option<Async>,
    is_managed: bool,
    dependencies: Vec<ChildDependency>,
}

#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
pub struct ChildDependency {
    ident: Ident,
    awaitness: Option<Await>,
}

impl From<processing::Container> for Container {
    fn from(container: processing::Container) -> Self {
        let processing::Container {
            attrs,
            self_ty,
            dependencies,
        } = container;

        let managed_dependencies: Vec<_> = dependencies
            .iter()
            .filter(|dep| dep.borrow().lifetime.is_managed())
            .cloned()
            .collect();

        let fields = get_struct_fields(&managed_dependencies);

        let constructors = get_struct_field_constructors(&managed_dependencies);

        let scope_constructors = get_new_scope_constructors(&managed_dependencies);

        let dependencies = dependencies
            .into_iter()
            .map(|d| d.borrow().clone().into())
            .collect();

        Self {
            attrs,
            self_ty,
            fields,
            constructors,
            scope_constructors,
            dependencies,
        }
    }
}

fn get_struct_fields(
    managed_dependencies: &[Rc<RefCell<processing::Dependency>>],
) -> Punctuated<Field, Token![,]> {
    if managed_dependencies.is_empty() {
        Default::default()
    } else {
        let fields: Vec<Field> = managed_dependencies
            .iter()
            .map(|dep| {
                let dep_ref = dep.borrow();
                let ident = &dep_ref.sig.ident;
                let field_ty = &dep_ref.field_ty;

                let wrapper_ty = match &dep_ref.lifetime {
                    Lifetime::Singleton(_) => {
                        if dep_ref.sig.asyncness.is_some() {
                            let once_cell_path: Path = parse_str(ASYNC_ONCE_CELL_PATH)
                                .expect("ASYNC_ONCE_CELL_PATH to be a path");
                            quote! { std::sync::Arc<#once_cell_path<#field_ty>> }
                        } else {
                            quote! { std::rc::Rc<std::cell::OnceCell<#field_ty>> }
                        }
                    }
                    Lifetime::Scoped(_) => {
                        if dep_ref.sig.asyncness.is_some() {
                            let once_cell_path: Path = parse_str(ASYNC_ONCE_CELL_PATH)
                                .expect("ASYNC_ONCE_CELL_PATH to be a path");
                            quote! { #once_cell_path<#field_ty> }
                        } else {
                            quote! { std::cell::OnceCell<#field_ty> }
                        }
                    }
                    Lifetime::Transient => {
                        unreachable!("we filtered for only singleton and scoped dependencies")
                    }
                };

                parse_quote! {
                    #ident: #wrapper_ty
                }
            })
            .collect();

        // Can't parse directly to punctuated because [Field] does not implement [Parse].
        // So using this workaround.
        let fields_named: FieldsNamed = parse_quote! {
            {
                #(#fields,)*
            }
        };

        fields_named.named
    }
}

fn get_struct_field_constructors(
    managed_dependencies: &[Rc<RefCell<processing::Dependency>>],
) -> Punctuated<FieldValue, Token![,]> {
    if managed_dependencies.is_empty() {
        Default::default()
    } else {
        let fields: Vec<FieldValue> = managed_dependencies
            .iter()
            .map(|dep| {
                let dep_ref = dep.borrow();
                let ident = &dep_ref.sig.ident;

                parse_quote! {
                    #ident: Default::default()
                }
            })
            .collect();

        parse_quote! { #(#fields,)* }
    }
}

fn get_new_scope_constructors(
    managed_dependencies: &[Rc<RefCell<processing::Dependency>>],
) -> Punctuated<FieldValue, Token![,]> {
    if managed_dependencies.is_empty() {
        Default::default()
    } else {
        let fields: Vec<FieldValue> = managed_dependencies
            .iter()
            .map(|dep| {
                let dep_ref = dep.borrow();
                let ident = &dep_ref.sig.ident;
                let init = match dep_ref.lifetime {
                    Lifetime::Singleton(_) => quote! { self.#ident.clone() },
                    Lifetime::Scoped(_) => quote! { Default::default() },
                    Lifetime::Transient => {
                        unreachable!("we filtered for only singleton and scoped dependencies")
                    }
                };

                parse_quote! {
                    #ident: #init
                }
            })
            .collect();

        parse_quote! { #(#fields,)* }
    }
}

impl From<processing::Dependency> for Dependency {
    fn from(dependency: processing::Dependency) -> Self {
        let processing::Dependency {
            attrs,
            sig,
            block,
            is_async,
            is_boxed: _,
            has_explicit_lifetime: _,
            lifetime,
            ty,
            field_ty: _,
            dependencies,
        } = dependency;

        let Signature {
            constness: _,
            asyncness,
            unsafety: _,
            abi: _,
            fn_token,
            ident,
            generics: _,
            paren_token,
            inputs,
            variadic: _,
            output: _,
        } = sig;

        let create_asyncness = asyncness;

        let asyncness = if is_async {
            Some(<Token![async]>::default())
        } else {
            None
        };

        let is_managed = lifetime.is_managed();

        let ty = if is_managed { parse_quote!(&#ty) } else { ty };

        let dependencies = dependencies
            .into_iter()
            .map(ChildDependency::from)
            .collect();

        Self {
            create_asyncness,
            attrs,
            block,
            asyncness,
            fn_token,
            ident,
            paren_token,
            inputs,
            ty,
            is_managed,
            dependencies,
        }
    }
}

impl From<processing::ChildDependency> for ChildDependency {
    fn from(child_dependency: processing::ChildDependency) -> Self {
        let dep_ref = child_dependency.inner.borrow();
        let ident = dep_ref.sig.ident.clone();
        let awaitness = if dep_ref.is_async {
            Some(<Token![await]>::default())
        } else {
            None
        };

        Self { ident, awaitness }
    }
}

impl ToTokens for Container {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            attrs,
            self_ty,
            fields,
            constructors,
            scope_constructors,
            dependencies,
        } = self;

        tokens.extend(quote! {
            #(#attrs)*
            struct #self_ty <'a> {
                #fields
                _phantom: std::marker::PhantomData<&'a ()>,
            }

            impl <'a> #self_ty <'a> {
                pub fn new() -> Self {
                    Self {
                        #constructors
                        _phantom: Default::default(),
                    }
                }

                pub fn new_scope(&self) -> Self {
                    Self {
                        #scope_constructors
                        _phantom: Default::default(),
                    }
                }

                #(#dependencies)*
            }
        });
    }
}

impl ToTokens for Dependency {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            attrs,
            block,
            asyncness,
            fn_token,
            ident,
            paren_token,
            inputs,
            ty,
            create_asyncness,
            is_managed,
            dependencies,
        } = self;

        // Do the same thing `syn` does for the paren_token
        let mut params = TokenStream::new();

        paren_token.surround(&mut params, |tokens| {
            inputs.to_tokens(tokens);
        });

        let create_dependencies: Vec<_> = dependencies
            .iter()
            .map(|child_dependency| {
                let ChildDependency { ident, awaitness } = child_dependency;

                let awaitness = awaitness.map(|awaitness| quote! { .#awaitness });

                let create_stmt = quote! {
                    let #ident = self.#ident()#awaitness;
                };

                create_stmt
            })
            .collect();

        // Figure out the correct final statement
        let final_stmt = if *is_managed {
            if create_asyncness.is_some() {
                quote! {
                    self.#ident.get_or_init(async { #block }).await
                }
            } else {
                quote! {
                    self.#ident.get_or_init(|| #block)
                }
            }
        } else {
            quote! {
                #block
            }
        };

        tokens.extend(quote!(
            #(#attrs)*
            pub #asyncness #fn_token #ident(&self) -> #ty {
                #(#create_dependencies)*

                #final_stmt
            }
        ));
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use pretty_assertions::assert_eq;
    use proc_macro2::Span;
    use syn::parse_quote;

    use crate::processing::{self, Lifetime};

    use super::*;

    #[test]
    fn from_processing_container() {
        let config = Rc::new(RefCell::new(processing::Dependency {
            attrs: vec![],
            sig: parse_quote! {
                async fn config(&self) -> Config
            },
            block: parse_quote!({ Config::new().await }),
            is_async: true,
            is_boxed: false,
            has_explicit_lifetime: false,
            lifetime: Lifetime::Singleton(Span::call_site()),
            ty: parse_quote! { Config },
            field_ty: Some(parse_quote! { Config }),
            dependencies: vec![],
        }));
        let db = Rc::new(RefCell::new(processing::Dependency {
            attrs: vec![],
            sig: parse_quote! {
                fn db(&self, config: &Config) -> Sqlite
            },
            block: parse_quote!({ Sqlite::new(config.conn_str) }),
            is_async: true,
            is_boxed: false,
            has_explicit_lifetime: false,
            lifetime: Lifetime::Singleton(Span::call_site()),
            ty: parse_quote! { Sqlite },
            field_ty: Some(parse_quote! { Sqlite }),
            dependencies: vec![processing::ChildDependency {
                inner: config.clone(),
                ty: parse_quote!(&Config),
            }],
        }));
        let container = processing::Container {
            attrs: vec![],
            self_ty: parse_quote! { Container },
            dependencies: vec![
                config,
                db.clone(),
                Rc::new(RefCell::new(processing::Dependency {
                    attrs: vec![],
                    sig: parse_quote! {
                        fn service(&self, db: &Sqlite) -> Service
                    },
                    block: parse_quote!({ Service::new(db) }),
                    is_async: true,
                    is_boxed: false,
                    has_explicit_lifetime: false,
                    lifetime: Lifetime::Transient,
                    ty: parse_quote! { Service },
                    field_ty: None,
                    dependencies: vec![processing::ChildDependency {
                        inner: db,
                        ty: parse_quote!(&Sqlite),
                    }],
                })),
            ],
        };
        let container: super::Container = container.into();

        let fields = {
            let named: FieldsNamed = parse_quote! {
                {
                    config: std::sync::Arc<async_once_cell::OnceCell<Config>>,
                    db: std::rc::Rc<std::cell::OnceCell<Sqlite>>,
                }
            };
            named.named
        };
        let expected = super::Container {
            attrs: vec![],
            self_ty: parse_quote! { Container },
            fields,
            constructors: parse_quote!( config: Default::default(), db: Default::default(), ),
            scope_constructors: parse_quote!( config: self.config.clone(), db: self.db.clone(), ),
            dependencies: vec![
                Dependency {
                    attrs: vec![],
                    block: parse_quote!({ Config::new().await }),
                    asyncness: Some(parse_quote!(async)),
                    fn_token: parse_quote!(fn),
                    ident: parse_quote!(config),
                    paren_token: Default::default(),
                    inputs: parse_quote!(&self),
                    ty: parse_quote!(&Config),
                    create_asyncness: Some(parse_quote!(async)),
                    is_managed: true,
                    dependencies: vec![],
                },
                Dependency {
                    attrs: vec![],
                    block: parse_quote!({ Sqlite::new(config.conn_str) }),
                    asyncness: Some(parse_quote!(async)),
                    fn_token: parse_quote!(fn),
                    ident: parse_quote!(db),
                    paren_token: Default::default(),
                    inputs: parse_quote!(&self, config: &Config),
                    ty: parse_quote!(&Sqlite),
                    create_asyncness: None,
                    is_managed: true,
                    dependencies: vec![ChildDependency {
                        ident: parse_quote!(config),
                        awaitness: Some(parse_quote!(await)),
                    }],
                },
                Dependency {
                    attrs: vec![],
                    block: parse_quote!({ Service::new(db) }),
                    asyncness: Some(parse_quote!(async)),
                    fn_token: parse_quote!(fn),
                    ident: parse_quote!(service),
                    paren_token: Default::default(),
                    inputs: parse_quote!(&self, db: &Sqlite),
                    ty: parse_quote!(Service),
                    create_asyncness: None,
                    is_managed: false,
                    dependencies: vec![ChildDependency {
                        ident: parse_quote!(db),
                        awaitness: Some(parse_quote!(await)),
                    }],
                },
            ],
        };

        assert_eq!(container, expected);
    }

    #[test]
    fn from_processing_dependency() {
        let dependency = processing::Dependency {
            attrs: vec![],
            sig: parse_quote! {
                fn db(&self) -> Box<dyn DB>
            },
            block: parse_quote!({ Box::new(Sqlite::new()) }),
            is_async: false,
            is_boxed: true,
            has_explicit_lifetime: false,
            lifetime: Lifetime::Scoped(Span::call_site()),
            ty: parse_quote! { std::boxed::Box<dyn DB + 'a> },
            field_ty: Some(parse_quote! { std::boxed::Box<dyn DB + 'a> }),
            dependencies: vec![],
        };
        let dependency: Dependency = dependency.into();

        let expected = Dependency {
            attrs: vec![],
            block: parse_quote!({ Box::new(Sqlite::new()) }),
            asyncness: None,
            fn_token: parse_quote!(fn),
            ident: parse_quote!(db),
            paren_token: Default::default(),
            inputs: parse_quote!(&self),
            ty: parse_quote!(&std::boxed::Box<dyn DB + 'a>),
            create_asyncness: None,
            is_managed: true,
            dependencies: vec![],
        };

        assert_eq!(dependency, expected);
    }
}
