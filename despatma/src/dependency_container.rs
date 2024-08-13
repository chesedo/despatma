use indexmap::IndexMap;
use proc_macro2::TokenStream;
use proc_macro_error::emit_error;
use quote::{quote, ToTokens};
use strsim::levenshtein;
use syn::{Block, Ident, ImplItem, ItemImpl, Signature, Type};

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
        validate_dependencies(&self.dependencies);
    }
}

fn validate_dependencies(dependencies: &IndexMap<Ident, Dependency>) {
    // Check if all dependencies are present
    for dep in dependencies.values() {
        for child_dep in &dep.dependencies {
            if !dependencies.contains_key(&child_dep.ident) {
                if let Some(best_match) =
                    get_best_dependency_match(dependencies, &child_dep.ident.to_string())
                {
                    emit_error!(child_dep.ident, "The '{}' dependency has not been registered", child_dep.ident; hint = best_match.span() => format!("Did you mean `{}`?", best_match));
                } else {
                    emit_error!(
                        child_dep.ident,
                        "Dependency not found. Did you forget to add it?";
                        hint = "Try adding it with `fn {}(&self) ...`", child_dep.ident
                    );
                }
            }
        }
    }
}

const MISSPELLING_THRESHOLD: usize = 3;

fn get_best_dependency_match<'a>(
    dependencies: &'a IndexMap<Ident, Dependency>,
    needle: &str,
) -> Option<&'a Ident> {
    dependencies
        .keys()
        .map(|d| (d, levenshtein(&needle.to_string(), &d.to_string())))
        .filter(|(_, distance)| *distance <= MISSPELLING_THRESHOLD)
        .min_by_key(|(_, distance)| *distance)
        .map(|(d, _)| d)
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
