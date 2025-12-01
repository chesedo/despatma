use std::{cell::RefCell, rc::Rc};

use crate::processing::{self, Lifetime};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::token::Comma;
use syn::{
    parse_quote, parse_str,
    punctuated::Punctuated,
    token::{Async, Fn, Paren},
    Attribute, Block, Field, FieldValue, FieldsNamed, FnArg, Ident, PatType, Path, Signature, Stmt,
    Token, Type, Visibility,
};

#[cfg(any(test, feature = "standalone"))]
const ASYNC_ONCE_CELL_PATH: &str = "async_once_cell::OnceCell";

#[cfg(not(any(test, feature = "standalone")))]
const ASYNC_ONCE_CELL_PATH: &str = "despatma::async_once_cell::OnceCell";

#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
pub struct Container {
    vis: Visibility,
    attrs: Vec<Attribute>,
    self_ty: Type,
    constructor_arguments: Punctuated<FnArg, Comma>,
    fields: Punctuated<Field, Token![,]>,
    constructors: Punctuated<FieldValue, Token![,]>,
    scope_constructors: Punctuated<FieldValue, Token![,]>,
    dependencies: Vec<Dependency>,
}

#[cfg_attr(test, derive(Eq, PartialEq, Debug, Clone))]
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
    is_embedded: bool,
    dependencies: Vec<Dependency>,
}

impl From<processing::Container> for Container {
    fn from(container: processing::Container) -> Self {
        let processing::Container {
            vis,
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

        let constructor_arguments = get_constructor_arguments(&managed_dependencies);

        let dependencies = dependencies
            .into_iter()
            .map(|d| d.borrow().clone().into())
            .collect();

        Self {
            vis,
            attrs,
            self_ty,
            constructor_arguments,
            fields,
            constructors,
            scope_constructors,
            dependencies,
        }
    }
}

fn get_constructor_arguments(
    managed_dependencies: &[Rc<RefCell<processing::Dependency>>],
) -> Punctuated<FnArg, Token![,]> {
    let embedded_dependencies: Vec<_> = managed_dependencies
        .iter()
        .filter(|d| d.borrow().lifetime.is_embedded())
        .map(|d| {
            let dep_ref = d.borrow();
            let ident = &dep_ref.sig.ident;
            let field_ty = &dep_ref.field_ty;

            let pt: PatType = parse_quote! {
                #ident: #field_ty
            };
            pt
        })
        .collect();

    if embedded_dependencies.is_empty() {
        return Default::default();
    }

    parse_quote!(#(#embedded_dependencies,)*)
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
                    Lifetime::Singleton(_) | Lifetime::Scoped(_) => {
                        if dep_ref.sig.asyncness.is_some() {
                            let once_cell_path: Path = parse_str(ASYNC_ONCE_CELL_PATH)
                                .expect("ASYNC_ONCE_CELL_PATH to be a path");
                            quote! { std::sync::Arc<#once_cell_path<#field_ty>> }
                        } else {
                            quote! { std::rc::Rc<std::cell::OnceCell<#field_ty>> }
                        }
                    }
                    Lifetime::Embedded(_) => quote! { std::sync::Arc<#field_ty> },
                    Lifetime::Transient(_) => {
                        unreachable!(
                            "we filtered for only singleton, scoped and embedded dependencies"
                        )
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

                match dep_ref.lifetime {
                    Lifetime::Singleton(_) | Lifetime::Scoped(_) => parse_quote! {
                        #ident: Default::default()
                    },
                    Lifetime::Embedded(_) => parse_quote! {
                        #ident: std::sync::Arc::new(#ident)
                    },
                    Lifetime::Transient(_) => unreachable!(
                        "we filtered for only singleton, scoped and embedded dependencies"
                    ),
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
                    Lifetime::Singleton(_) | Lifetime::Embedded(_) => {
                        quote! { self.#ident.clone() }
                    }
                    Lifetime::Scoped(_) => quote! { Default::default() },
                    Lifetime::Transient(_) => {
                        unreachable!(
                            "we filtered for only singleton, scoped and embedded dependencies"
                        )
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
        let is_embedded = lifetime.is_embedded();
        let ty = if is_managed { parse_quote!(&#ty) } else { ty };

        let dependencies = dependencies
            .into_iter()
            .map(|d| d.inner.borrow().clone().into())
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
            is_embedded,
            dependencies,
        }
    }
}

impl ToTokens for Container {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            vis,
            attrs,
            self_ty,
            constructor_arguments,
            fields,
            constructors,
            scope_constructors,
            dependencies,
        } = self;

        tokens.extend(quote! {
            #(#attrs)*
            #[derive(core::clone::Clone)]
            #vis struct #self_ty <'a> {
                #fields
                _phantom: std::marker::PhantomData<&'a ()>,
            }

            impl <'a> #self_ty <'a> {
                pub fn new(#constructor_arguments) -> Self {
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
            block: _,
            asyncness,
            fn_token,
            ident,
            paren_token,
            inputs,
            ty,
            create_asyncness: _,
            is_managed: _,
            is_embedded: _,
            dependencies: _,
        } = self;

        // Do the same thing `syn` does for the paren_token
        let mut params = TokenStream::new();

        paren_token.surround(&mut params, |tokens| {
            inputs.to_tokens(tokens);
        });

        let stmts = self.to_stmts();

        tokens.extend(quote!(
            #(#attrs)*
            pub #asyncness #fn_token #ident(&'a self) -> #ty {
                #(#stmts);*
            }
        ));
    }
}

impl Dependency {
    fn to_stmts(&self) -> Vec<Stmt> {
        let Self {
            attrs: _,
            block,
            asyncness: _,
            fn_token: _,
            ident,
            paren_token: _,
            inputs: _,
            ty: _,
            create_asyncness,
            is_managed,
            is_embedded,
            dependencies,
        } = self;

        let create_dependencies: Vec<_> = dependencies
            .iter()
            .map(|child_dependency| {
                let stmts = child_dependency.to_stmts();
                let ident = &child_dependency.ident;

                let block = if stmts.len() == 1 {
                    let stmt = &stmts[0];

                    quote! { #stmt }
                } else {
                    quote! { {  #(#stmts);* } }
                };

                let create_stmt = quote! {
                    let #ident = #block;
                };

                create_stmt
            })
            .collect();

        // Figure out the correct final statement
        let final_stmt = if *is_managed && !is_embedded {
            if create_asyncness.is_some() {
                quote! {
                    self.#ident.get_or_init(async #block ).await
                }
            } else {
                quote! {
                    self.#ident.get_or_init(|| #block)
                }
            }
        } else if *is_embedded {
            quote! {
                self.#ident.as_ref()
            }
        } else {
            let stmts = &block.stmts;
            quote! {
                #(#stmts)*
            }
        };

        parse_quote! {
            #(#create_dependencies)*

            #final_stmt
        }
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
        let embedded = Rc::new(RefCell::new(processing::Dependency {
            attrs: vec![],
            sig: parse_quote!(fn embedded(&self) -> Embedded),
            block: parse_quote!({}),
            is_async: false,
            is_boxed: false,
            lifetime: Lifetime::Embedded(Span::call_site()),
            ty: parse_quote! { Embedded },
            field_ty: parse_quote! { Embedded },
            dependencies: vec![],
        }));

        let config = Rc::new(RefCell::new(processing::Dependency {
            attrs: vec![],
            sig: parse_quote! {
                async fn config(&self) -> Config
            },
            block: parse_quote!({ Config::new().await }),
            is_async: true,
            is_boxed: false,
            lifetime: Lifetime::Singleton(Span::call_site()),
            ty: parse_quote! { Config },
            field_ty: parse_quote! { Config },
            dependencies: vec![],
        }));
        let db = Rc::new(RefCell::new(processing::Dependency {
            attrs: vec![],
            sig: parse_quote! {
                fn db(&self, config: &Config, embedded: &Embedded) -> Sqlite
            },
            block: parse_quote!({ Sqlite::new(config.conn_str, embedded.some_val) }),
            is_async: true,
            is_boxed: false,
            lifetime: Lifetime::Singleton(Span::call_site()),
            ty: parse_quote! { Sqlite },
            field_ty: parse_quote! { Sqlite },
            dependencies: vec![
                processing::ChildDependency {
                    inner: config.clone(),
                    ty: parse_quote!(&Config),
                },
                processing::ChildDependency {
                    inner: embedded.clone(),
                    ty: parse_quote!(&Embedded),
                },
            ],
        }));
        let container = processing::Container {
            vis: syn::Visibility::Inherited,
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
                    lifetime: Lifetime::Transient(None),
                    ty: parse_quote! { Service },
                    field_ty: parse_quote! { Service },
                    dependencies: vec![processing::ChildDependency {
                        inner: db,
                        ty: parse_quote!(&Sqlite),
                    }],
                })),
                embedded,
            ],
        };
        let container: super::Container = container.into();

        let fields = {
            let named: FieldsNamed = parse_quote! {
                {
                    config: std::sync::Arc<async_once_cell::OnceCell<Config>>,
                    db: std::rc::Rc<std::cell::OnceCell<Sqlite>>,
                    embedded: std::sync::Arc<Embedded>,
                }
            };
            named.named
        };
        let config = Dependency {
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
            is_embedded: false,
            dependencies: vec![],
        };
        let embedded = Dependency {
            attrs: vec![],
            block: parse_quote!({}),
            asyncness: None,
            fn_token: parse_quote!(fn),
            ident: parse_quote!(embedded),
            paren_token: Default::default(),
            ty: parse_quote! { &Embedded },
            create_asyncness: None,
            dependencies: vec![],
            inputs: parse_quote!(&self),
            is_managed: true,
            is_embedded: true,
        };
        let db = Dependency {
            attrs: vec![],
            block: parse_quote!({ Sqlite::new(config.conn_str, embedded.some_val) }),
            asyncness: Some(parse_quote!(async)),
            fn_token: parse_quote!(fn),
            ident: parse_quote!(db),
            paren_token: Default::default(),
            inputs: parse_quote!(&self, config: &Config, embedded: &Embedded),
            ty: parse_quote!(&Sqlite),
            create_asyncness: None,
            is_managed: true,
            is_embedded: false,
            dependencies: vec![config.clone(), embedded.clone()],
        };
        let expected = super::Container {
            vis: Visibility::Inherited,
            attrs: vec![],
            self_ty: parse_quote! { Container },
            constructor_arguments: parse_quote!(embedded: Embedded,),
            fields,
            constructors: parse_quote!( config: Default::default(), db: Default::default(), embedded: std::sync::Arc::new(embedded), ),
            scope_constructors: parse_quote!( config: self.config.clone(), db: self.db.clone(), embedded: self.embedded.clone(), ),
            dependencies: vec![
                config,
                db.clone(),
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
                    is_embedded: false,
                    dependencies: vec![db],
                },
                embedded,
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
            lifetime: Lifetime::Scoped(Span::call_site()),
            ty: parse_quote! { std::boxed::Box<dyn DB + 'a> },
            field_ty: parse_quote! { std::boxed::Box<dyn DB + 'a> },
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
            is_embedded: false,
            dependencies: vec![],
        };

        assert_eq!(dependency, expected);
    }
}
