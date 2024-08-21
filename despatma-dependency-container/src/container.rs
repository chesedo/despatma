use std::{cell::RefCell, rc::Rc};

use indexmap::IndexMap;
use proc_macro2::{Span, TokenStream};
use proc_macro_error::emit_error;
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse_quote, spanned::Spanned, Attribute, Block, FnArg, Ident, ImplItem, ImplItemFn, ItemImpl,
    Meta, Pat, ReturnType, Signature, Token, Type,
};

use crate::visitor::{
    CheckWiring, ErrorVisitor, FixAsyncTree, ImplTraitButRegisteredConcrete, SetOutputSpans,
    Visitable, VisitableMut,
};

#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
pub struct Container {
    pub attrs: Vec<Attribute>,
    pub self_ty: Type,
    pub dependencies: IndexMap<Ident, Rc<RefCell<Dependency>>>,
}

impl Container {
    pub fn from_item_impl(item_impl: ItemImpl) -> Self {
        let dependencies = item_impl
            .items
            .into_iter()
            .filter_map(|impl_item| match impl_item {
                ImplItem::Fn(impl_item_fn) => Some((
                    impl_item_fn.sig.ident.clone(),
                    Rc::new(RefCell::new(Dependency::from_impl_item_fn(impl_item_fn))),
                )),
                impl_item => {
                    emit_error!(impl_item, "This impl item is not supported");
                    None
                }
            })
            .collect();

        Self {
            attrs: item_impl.attrs,
            self_ty: item_impl.self_ty.as_ref().clone(),
            dependencies,
        }
    }

    /// Validate the supplied input is correct
    pub fn validate(&self) {
        self.validate_wiring();
        self.validate_rpit_requesting_concrete();
    }

    fn validate_wiring(&self) {
        let mut wiring_visitor = CheckWiring::new(self.dependencies.keys().cloned().collect());

        self.apply(&mut wiring_visitor);
        wiring_visitor.emit_errors();
    }

    fn validate_rpit_requesting_concrete(&self) {
        let mut rpit_requesting_concrete =
            ImplTraitButRegisteredConcrete::new(self.dependencies.clone());
        self.apply(&mut rpit_requesting_concrete);
        rpit_requesting_concrete.emit_errors();
    }

    /// Update the container to fix any issues
    pub fn update(&mut self) {
        let mut async_visitor = FixAsyncTree::new(self.dependencies.clone());
        self.apply_mut(&mut async_visitor);

        let mut set_output_span = SetOutputSpans::new(self.dependencies.clone());
        self.apply_mut(&mut set_output_span);
    }
}

impl ToTokens for Container {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let self_attrs = &self.attrs;
        let self_ty = &self.self_ty;
        let self_dependencies = &self.dependencies;

        let singleton_and_scoped_dependencies: Vec<_> = self_dependencies
            .values()
            .filter(|dep| {
                matches!(
                    dep.borrow().lifetime,
                    Lifetime::Singleton | Lifetime::Scoped
                )
            })
            .cloned()
            .collect();

        let struct_fields = if singleton_and_scoped_dependencies.is_empty() {
            quote! {;}
        } else {
            let fields = singleton_and_scoped_dependencies.iter().map(|dep| {
                let dep_ref = dep.borrow();
                let ident = &dep_ref.sig.ident;
                let ty = &dep_ref.ty;
                let wrapper_ty = match dep_ref.lifetime {
                    Lifetime::Singleton => quote! { std::rc::Rc<std::cell::OnceCell<#ty>> },
                    Lifetime::Scoped => quote! { std::cell::OnceCell<#ty> },
                    Lifetime::Transient => {
                        unreachable!("we filtered for only singleton and scoped dependencies")
                    }
                };

                quote! {
                    #ident: #wrapper_ty,
                }
            });

            quote! {
                {
                    #(#fields)*
                }
            }
        };

        let fields_contructors = if singleton_and_scoped_dependencies.is_empty() {
            quote! {}
        } else {
            let fields = singleton_and_scoped_dependencies.iter().map(|dep| {
                let dep_ref = dep.borrow();
                let ident = &dep_ref.sig.ident;

                quote! {
                    #ident: Default::default(),
                }
            });

            quote! {
                {
                    #(#fields)*
                }
            }
        };

        let new_scope_contructors = if singleton_and_scoped_dependencies.is_empty() {
            quote! {}
        } else {
            let fields = singleton_and_scoped_dependencies.iter().map(|dep| {
                let dep_ref = dep.borrow();
                let ident = &dep_ref.sig.ident;
                let init = match dep_ref.lifetime {
                    Lifetime::Singleton => quote! { self.#ident.clone() },
                    Lifetime::Scoped => quote! { Default::default() },
                    Lifetime::Transient => {
                        unreachable!("we filtered for only singleton and scoped dependencies")
                    }
                };

                quote! {
                    #ident: #init,
                }
            });

            quote! {
                {
                    #(#fields)*
                }
            }
        };

        let dependencies = self_dependencies.values().map(|dep| {
            let dep_ref = dep.borrow();

            let (create_ident, create_dependency_fn) = dep_ref.create_dependency_fn();
            let dependency_fn = dep_ref.dependency_fn(create_ident, self_dependencies);

            quote! {
                #create_dependency_fn

                #dependency_fn
            }
        });

        tokens.extend(quote! {
            #(#self_attrs)*
            struct #self_ty #struct_fields

            impl #self_ty {
                fn new() -> Self {
                    Self #fields_contructors
                }

                pub fn new_scope(&self) -> Self {
                    Self #new_scope_contructors
                }

                #(#dependencies)*
            }
        });
    }
}

#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
pub struct Dependency {
    pub attrs: Vec<Attribute>,
    pub sig: Signature,
    pub block: Block,
    pub is_async: bool,
    pub lifetime: Lifetime,
    pub ty: Type,
    pub dependencies: Vec<ChildDependency>,
}

#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
pub enum Lifetime {
    Transient,
    Scoped,
    Singleton,
}

impl Dependency {
    fn from_impl_item_fn(mut impl_item_fn: ImplItemFn) -> Self {
        let dependencies = ChildDependency::from_impl_item_fn(&impl_item_fn);
        let mut lifetime = Lifetime::Transient;

        // Remove all lifetime attributes
        impl_item_fn.attrs.retain(|attr| {
            let Meta::Path(ref path) = attr.meta else {
                return true;
            };

            if path.segments.len() == 1 {
                match path.segments[0].ident.to_string().as_str() {
                    "Transient" => {
                        lifetime = Lifetime::Transient;
                        false
                    }
                    "Scoped" => {
                        lifetime = Lifetime::Scoped;
                        false
                    }

                    "Singleton" => {
                        lifetime = Lifetime::Singleton;
                        false
                    }
                    _ => true,
                }
            } else {
                true
            }
        });

        let ty = match &impl_item_fn.sig.output {
            ReturnType::Type(_, ty) => ty.as_ref().clone(),
            ReturnType::Default => parse_quote! { () },
        };

        Self {
            attrs: impl_item_fn.attrs,
            is_async: impl_item_fn.sig.asyncness.is_some(),
            sig: impl_item_fn.sig,
            block: impl_item_fn.block,
            lifetime,
            ty,
            dependencies,
        }
    }

    fn create_dependency_fn(&self) -> (Ident, TokenStream) {
        let Self {
            attrs: _,
            sig,
            block,
            is_async: _,
            lifetime: _,
            ty,
            dependencies: _,
        } = self;
        let Signature {
            constness,
            asyncness,
            unsafety,
            abi,
            fn_token,
            ident,
            generics,
            paren_token,
            inputs,
            variadic,
            output: _,
        } = sig;

        let create_ident = Ident::new(&format!("create_{}", ident), ident.span());

        // Do the same thing `syn` does for the paren_token
        let mut params = TokenStream::new();

        paren_token.surround(&mut params, |tokens| {
            inputs.to_tokens(tokens);
            if let Some(variadic) = &variadic {
                if !inputs.empty_or_trailing() {
                    <Token![,]>::default().to_tokens(tokens);
                }
                variadic.to_tokens(tokens);
            }
        });

        let create_fn = quote! {
            #constness #asyncness #unsafety #abi #fn_token #create_ident #generics #params -> #ty #block
        };

        (create_ident, create_fn)
    }

    fn dependency_fn(
        &self,
        create_ident: Ident,
        container_dependencies: &IndexMap<Ident, Rc<RefCell<Dependency>>>,
    ) -> TokenStream {
        let Dependency {
            attrs,
            sig,
            block: _,
            is_async,
            lifetime,
            ty,
            dependencies,
        } = self;
        let Signature {
            constness,
            asyncness,
            unsafety,
            abi,
            fn_token,
            ident,
            generics,
            paren_token: _,
            inputs: _,
            variadic: _,
            output: _,
        } = sig;

        let (create_dependencies, dependency_params): (Vec<_>, Vec<_>) = dependencies
            .iter()
            .map(|child_dependency| {
                let ChildDependency {
                    ident,
                    is_ref,
                    request_ty_span,
                    registered_ty_span,
                } = child_dependency;

                // The dependency might not exist if it was mispelt since we still try our best to generate the code.
                // So defaulting to false
                let is_async = container_dependencies
                    .get(ident)
                    .map(|d| d.borrow().is_async)
                    .unwrap_or_default();

                // The dependency might not exist if it was mispelt since we still try our best to generate the code.
                // So defaulting to true
                let is_transient = !container_dependencies
                    .get(ident)
                    .map(|d| matches!(d.borrow().lifetime, Lifetime::Scoped | Lifetime::Singleton))
                    .unwrap_or_default();

                let param = if *is_ref && is_transient {
                    // Need to do some weird stuff to get the compile errors for type mismatches to appear correctly
                    // Feel free to update this if the mismatch error can be improved
                    let ident_request_ty_spanned =
                        Ident::new(&format!("{}", ident), *request_ty_span);
                    quote_spanned! { *registered_ty_span => &#ident_request_ty_spanned }
                } else {
                    // Need to do some weird stuff to get the compile errors for type mismatches to appear correctly
                    // Feel free to update this if the mismatch error can be improved
                    let ident_output_spanned =
                        Ident::new(&format!("{}", ident), *registered_ty_span);
                    quote_spanned! { *request_ty_span => #ident_output_spanned }
                };

                let await_key = if is_async {
                    Some(quote! { .await })
                } else {
                    None
                };

                let create_stmt = quote! {
                    let #ident = self.#ident()#await_key;
                };

                (create_stmt, param)
            })
            .unzip();

        let async_key = if *is_async {
            Some(<Token![async]>::default())
        } else {
            None
        };
        let await_key = if asyncness.is_some() {
            Some(quote! { .await })
        } else {
            None
        };

        let create_call = quote! {
            self.#create_ident(#(#dependency_params),*)#await_key
        };

        let final_call = match lifetime {
            Lifetime::Transient => create_call,
            Lifetime::Scoped | Lifetime::Singleton => {
                quote! {
                    self.#ident.get_or_init(|| #create_call)
                }
            }
        };

        let final_output = match lifetime {
            Lifetime::Transient => quote! { -> #ty },
            Lifetime::Scoped | Lifetime::Singleton => {
                quote! { -> &#ty }
            }
        };

        quote! {
            #(#attrs)*
            pub #constness #async_key #unsafety #abi #fn_token #ident #generics(&self) #final_output {
                #(#create_dependencies)*

                #final_call
            }
        }
    }
}

#[cfg_attr(test, derive(Debug))]
pub struct ChildDependency {
    pub ident: Ident,
    pub is_ref: bool,
    pub request_ty_span: Span,
    pub registered_ty_span: Span,
}

// Manual impl since [Span] doesn't implement PartialEq
#[cfg(test)]
impl PartialEq for ChildDependency {
    fn eq(&self, other: &Self) -> bool {
        self.ident == other.ident && self.is_ref == other.is_ref
    }
}

#[cfg(test)]
impl Eq for ChildDependency {}

impl ChildDependency {
    fn from_impl_item_fn(impl_item_fn: &ImplItemFn) -> Vec<Self> {
        impl_item_fn
            .sig
            .inputs
            .iter()
            .filter_map(|fn_arg| match fn_arg {
                FnArg::Receiver(_) => None,
                FnArg::Typed(pat_type) => {
                    let ident = match pat_type.pat.as_ref() {
                        Pat::Ident(pat_ident) => pat_ident.ident.clone(),
                        pat => {
                            emit_error!(pat, "This argument type is not supported");
                            return None;
                        }
                    };
                    Some(Self {
                        ident,
                        is_ref: matches!(pat_type.ty.as_ref(), Type::Reference(_)),
                        request_ty_span: pat_type.ty.span(),
                        registered_ty_span: Span::call_site(),
                    })
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod inputs {
        use syn::parse_quote;

        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn simple() {
            let container = Container::from_item_impl(parse_quote!(
                impl DependencyContainer {
                    fn config(&self) -> Config {
                        Config
                    }
                }
            ));
            let expected = Container {
                attrs: vec![],
                self_ty: parse_quote!(DependencyContainer),
                dependencies: IndexMap::from_iter(vec![(
                    parse_quote!(config),
                    Rc::new(RefCell::new(Dependency {
                        attrs: vec![],
                        sig: parse_quote!(fn config(&self) -> Config),
                        block: parse_quote!({ Config }),
                        is_async: false,
                        lifetime: Lifetime::Transient,
                        ty: parse_quote!(Config),
                        dependencies: vec![],
                    })),
                )]),
            };

            assert_eq!(container, expected);
        }

        #[test]
        fn with_dependency() {
            let container = Container::from_item_impl(parse_quote!(
                impl Dependencies {
                    fn service(&self, config: Config) -> Service {
                        Service
                    }
                }
            ));
            let expected = Container {
                attrs: vec![],
                self_ty: parse_quote!(Dependencies),
                dependencies: IndexMap::from_iter(vec![(
                    parse_quote!(service),
                    Rc::new(RefCell::new(Dependency {
                        attrs: vec![],
                        sig: parse_quote!(fn service(&self, config: Config) -> Service),
                        block: parse_quote!({ Service }),
                        is_async: false,
                        lifetime: Lifetime::Transient,
                        ty: parse_quote!(Service),
                        dependencies: vec![ChildDependency {
                            ident: parse_quote!(config),
                            is_ref: false,
                            request_ty_span: Span::call_site(),
                            registered_ty_span: Span::call_site(),
                        }],
                    })),
                )]),
            };

            assert_eq!(container, expected);
        }

        #[test]
        fn with_ref_dependency() {
            let container = Container::from_item_impl(parse_quote!(
                impl Dependencies {
                    fn service(&self, config: &Config) -> Service {
                        Service
                    }
                }
            ));
            let expected = Container {
                attrs: vec![],
                self_ty: parse_quote!(Dependencies),
                dependencies: IndexMap::from_iter(vec![(
                    parse_quote!(service),
                    Rc::new(RefCell::new(Dependency {
                        attrs: vec![],
                        sig: parse_quote!(fn service(&self, config: &Config) -> Service),
                        block: parse_quote!({ Service }),
                        is_async: false,
                        lifetime: Lifetime::Transient,
                        ty: parse_quote!(Service),
                        dependencies: vec![ChildDependency {
                            ident: parse_quote!(config),
                            is_ref: true,
                            request_ty_span: Span::call_site(),
                            registered_ty_span: Span::call_site(),
                        }],
                    })),
                )]),
            };

            assert_eq!(container, expected);
        }

        #[test]
        fn with_async_dependency() {
            let container = Container::from_item_impl(parse_quote!(
                impl Dependencies {
                    fn service(&self, config: Config) -> Service {
                        Service
                    }
                    async fn config(&self) -> Config {
                        Config
                    }
                }
            ));
            let expected = Container {
                attrs: vec![],
                self_ty: parse_quote!(Dependencies),
                dependencies: IndexMap::from_iter(vec![
                    (
                        parse_quote!(service),
                        Rc::new(RefCell::new(Dependency {
                            attrs: vec![],
                            sig: parse_quote!(fn service(&self, config: Config) -> Service),
                            block: parse_quote!({ Service }),
                            is_async: false,
                            lifetime: Lifetime::Transient,
                            ty: parse_quote!(Service),
                            dependencies: vec![ChildDependency {
                                ident: parse_quote!(config),
                                is_ref: false,
                                request_ty_span: Span::call_site(),
                                registered_ty_span: Span::call_site(),
                            }],
                        })),
                    ),
                    (
                        parse_quote!(config),
                        Rc::new(RefCell::new(Dependency {
                            attrs: vec![],
                            sig: parse_quote!(async fn config(&self) -> Config),
                            block: parse_quote!({ Config }),
                            is_async: true,
                            lifetime: Lifetime::Transient,
                            ty: parse_quote!(Config),
                            dependencies: vec![],
                        })),
                    ),
                ]),
            };

            assert_eq!(container, expected);
        }

        #[test]
        fn with_lifetime() {
            let container = Container::from_item_impl(parse_quote!(
                impl DependencyContainer {
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
                }
            ));
            let expected = Container {
                attrs: vec![],
                self_ty: parse_quote!(DependencyContainer),
                dependencies: IndexMap::from_iter(vec![
                    (
                        parse_quote!(singleton),
                        Rc::new(RefCell::new(Dependency {
                            attrs: vec![],
                            sig: parse_quote!(fn singleton(&self) -> Singleton),
                            block: parse_quote!({ Singleton }),
                            is_async: false,
                            lifetime: Lifetime::Singleton,
                            ty: parse_quote!(Singleton),
                            dependencies: vec![],
                        })),
                    ),
                    (
                        parse_quote!(scoped),
                        Rc::new(RefCell::new(Dependency {
                            attrs: vec![],
                            sig: parse_quote!(fn scoped(&self) -> Scoped),
                            block: parse_quote!({ Scoped }),
                            is_async: false,
                            lifetime: Lifetime::Scoped,
                            ty: parse_quote!(Scoped),
                            dependencies: vec![],
                        })),
                    ),
                    (
                        parse_quote!(transient),
                        Rc::new(RefCell::new(Dependency {
                            attrs: vec![],
                            sig: parse_quote!(fn transient(&self) -> Transient),
                            block: parse_quote!({ Transient }),
                            is_async: false,
                            lifetime: Lifetime::Transient,
                            ty: parse_quote!(Transient),
                            dependencies: vec![],
                        })),
                    ),
                    (
                        parse_quote!(default),
                        Rc::new(RefCell::new(Dependency {
                            attrs: vec![],
                            sig: parse_quote!(fn default(&self) -> Default),
                            block: parse_quote!({ Default }),
                            is_async: false,
                            lifetime: Lifetime::Transient,
                            ty: parse_quote!(Default),
                            dependencies: vec![],
                        })),
                    ),
                ]),
            };

            assert_eq!(container, expected);
        }
    }
}
