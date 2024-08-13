use indexmap::IndexMap;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Block, Ident, ImplItem, ItemImpl, Signature, Type};

use self::visitor::{MutVisitor, WiringVisitor};

mod visitor;

#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
pub struct Container {
    self_ty: Type,
    dependencies: IndexMap<Ident, Dependency>,
}

#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
struct Dependency {
    sig: Signature,
    block: Block,
    dependencies: Vec<ChildDependency>,
}

#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
struct ChildDependency {
    ident: Ident,
    is_ref: bool,
}

impl Container {
    pub fn from_item_impl(item_impl: ItemImpl) -> Self {
        let dependencies = item_impl
            .items
            .into_iter()
            .filter_map(|item| match item {
                ImplItem::Fn(item_fn) => {
                    let dependencies = item_fn
                        .sig
                        .inputs
                        .iter()
                        .filter_map(|input| match input {
                            syn::FnArg::Receiver(_) => None,
                            syn::FnArg::Typed(pat_type) => {
                                let ident = match pat_type.pat.as_ref() {
                                    syn::Pat::Ident(pat_ident) => pat_ident.ident.clone(),
                                    _ => todo!(),
                                };
                                Some(ChildDependency {
                                    ident,
                                    is_ref: matches!(pat_type.ty.as_ref(), Type::Reference(_)),
                                })
                            }
                        })
                        .collect();

                    Some((
                        item_fn.sig.ident.clone(),
                        Dependency {
                            sig: item_fn.sig,
                            block: item_fn.block,
                            dependencies,
                        },
                    ))
                }
                _ => todo!(),
            })
            .collect();

        Self {
            self_ty: item_impl.self_ty.as_ref().clone(),
            dependencies,
        }
    }

    pub fn validate(&self) {
        let mut wiring_visitor = WiringVisitor::new(self.dependencies.keys().cloned().collect());

        wiring_visitor.visit_container(&self);
    }
}

impl ToTokens for Container {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            self_ty,
            dependencies,
        } = self;

        let dependencies = dependencies.values().map(|dep| {
            let Dependency {  sig, block, dependencies } = dep;
            let Signature {
                constness,
                asyncness,
                unsafety,
                abi,
                fn_token,
                ident,
                generics,
                paren_token: _,
                inputs,
                variadic,
                output,
            } = sig;

            let create_ident = Ident::new(&format!("create_{}", ident), ident.span());
            let (create_dependencies, dependency_params): (Vec<_>, Vec<_>) = dependencies.iter().map(|dep| {
                let ChildDependency { ident, is_ref } = dep;

                let param = if *is_ref {
                    quote! { &#ident }
                } else {
                    quote! { #ident }
                };

                (quote! {
                    let #ident = self.#ident();
                }, param)
            }).unzip();


            quote! {
                #constness #asyncness #unsafety #abi #fn_token #create_ident #generics (#inputs, #variadic) #output #block

                pub #constness #asyncness #unsafety #abi #fn_token #ident #generics(&self) #output {
                    #(#create_dependencies)*

                    self.#create_ident(#(#dependency_params),*)
                }
            }
        });

        tokens.extend(quote! {
            struct #self_ty;

            impl #self_ty {
                fn new() -> Self {
                    Self
                }

                #(#dependencies)*
            }
        });
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
                self_ty: parse_quote!(DependencyContainer),
                dependencies: IndexMap::from_iter(vec![(
                    parse_quote!(config),
                    Dependency {
                        sig: parse_quote!(fn config(&self) -> Config),
                        block: parse_quote!({ Config }),
                        dependencies: vec![],
                    },
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
                self_ty: parse_quote!(Dependencies),
                dependencies: IndexMap::from_iter(vec![(
                    parse_quote!(service),
                    Dependency {
                        sig: parse_quote!(fn service(&self, config: Config) -> Service),
                        block: parse_quote!({ Service }),
                        dependencies: vec![ChildDependency {
                            ident: parse_quote!(config),
                            is_ref: false,
                        }],
                    },
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
                self_ty: parse_quote!(Dependencies),
                dependencies: IndexMap::from_iter(vec![(
                    parse_quote!(service),
                    Dependency {
                        sig: parse_quote!(fn service(&self, config: &Config) -> Service),
                        block: parse_quote!({ Service }),
                        dependencies: vec![ChildDependency {
                            ident: parse_quote!(config),
                            is_ref: true,
                        }],
                    },
                )]),
            };

            assert_eq!(container, expected);
        }
    }
}
