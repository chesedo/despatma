use convert_case::{Case, Casing};
use despatma_lib::{AnnotatedType, KeyValue, SimpleType};
use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{Ident, Token};

/// Model for holding the input passed to the visitor macro
/// It expects a stream in the following format:
/// ```text
/// ConcreteType,
///
/// dyn DynamicType,
///
/// #[no_defuault]
/// NoDefault,
///
/// #[helper_tmpl = {visitor.visit_button(window.button);}]
/// CustomTemplate,
/// ```
///
/// Thus, it takes a list of types that will be visited.
/// A type can be concrete or dynamic.
///
/// Options can also be passed to the type:
/// - `no_default` to turn-off the defualt implementation for the trait method.
/// - 'helper_tmpl` to be filled into the helper template for traversing a types internal structure.
#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
pub struct VisitorFunction {
    types: Punctuated<AnnotatedType<SimpleType>, Token![,]>,
}

/// Make VisitorFunction parsable
impl Parse for VisitorFunction {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(VisitorFunction {
            types: input.parse_terminated(AnnotatedType::parse, Token![,])?,
        })
    }
}

impl VisitorFunction {
    /// Expand the visitor model into its implementation
    pub fn expand(&self) -> TokenStream {
        // Store each of the three parts
        let mut trait_functions: Vec<TokenStream> = Vec::new();
        let mut helpers: Vec<TokenStream> = Vec::new();
        let mut visitables: Vec<TokenStream> = Vec::new();

        // Loop over each type given
        for t in self.types.iter() {
            let elem_name = Ident::new(
                &t.inner_type.ident.to_string().to_case(Case::Snake),
                t.inner_type.ident.span(),
            );
            let elem_type = &t.inner_type;
            let fn_name = format_ident!("visit_{}", elem_name);
            let options = Options::new(&t.attrs.options);

            // Get trait function
            if options.no_default {
                trait_functions.push(quote! {
                    fn #fn_name(&mut self, #elem_name: &#elem_type);
                })
            } else {
                trait_functions.push(quote! {
                    fn #fn_name(&mut self, #elem_name: &#elem_type) {
                        #fn_name(self, #elem_name)
                    }
                })
            };

            // Get helper function
            if options.has_helper {
                if let Some(inner) = options.helper_tmpl {
                    helpers.push(quote! {
                        pub fn #fn_name<V>(visitor: &mut V, #elem_name: &#elem_type)
                        where
                            V: Visitor + ?Sized,
                        {
                            #inner
                        }
                    });
                } else {
                    let unused_elem_name = format_ident!("_{}", elem_name);
                    helpers.push(quote! {
                        pub fn #fn_name<V>(_visitor: &mut V, #unused_elem_name: &#elem_type)
                        where
                            V: Visitor + ?Sized,
                        {
                        }
                    });
                }
            };

            // Make visitable
            visitables.push(quote! {
                impl Visitable for #elem_type {
                    fn apply(&self, visitor: &mut impl Visitor) {
                        visitor.#fn_name(self);
                    }
                }
            });
        }

        // Built complete visitor implementation
        quote! {
            pub trait Visitor {
                #(#trait_functions)*
            }

            #(#helpers)*

            trait Visitable {
                fn apply(&self, visitor: &mut impl Visitor);
            }
            #(#visitables)*
        }
    }

    /// Expand the visitor model into a mutable implementation
    pub fn expand_mut(&self) -> TokenStream {
        // Store each of the three parts
        let mut trait_functions: Vec<TokenStream> = Vec::new();
        let mut helpers: Vec<TokenStream> = Vec::new();
        let mut visitables: Vec<TokenStream> = Vec::new();

        // Loop over each type given
        for t in self.types.iter() {
            let elem_name = Ident::new(
                &t.inner_type.ident.to_string().to_case(Case::Snake),
                t.inner_type.ident.span(),
            );
            let elem_type = &t.inner_type;
            let fn_name = format_ident!("visit_{}_mut", elem_name);
            let options = Options::new(&t.attrs.options);

            // Get trait function
            if options.no_default {
                trait_functions.push(quote! {
                    fn #fn_name(&mut self, #elem_name: &mut #elem_type);
                })
            } else {
                trait_functions.push(quote! {
                    fn #fn_name(&mut self, #elem_name: &mut #elem_type) {
                        #fn_name(self, #elem_name)
                    }
                })
            };

            // Get helper function
            if options.has_helper {
                if let Some(inner) = options.helper_tmpl {
                    helpers.push(quote! {
                        pub fn #fn_name<V>(visitor: &mut V, #elem_name: &mut #elem_type)
                        where
                            V: VisitorMut + ?Sized,
                        {
                            #inner
                        }
                    });
                } else {
                    let unused_elem_name = format_ident!("_{}", elem_name);
                    helpers.push(quote! {
                        pub fn #fn_name<V>(_visitor: &mut V, #unused_elem_name: &mut #elem_type)
                        where
                            V: VisitorMut + ?Sized,
                        {
                        }
                    });
                }
            };

            // Make visitable
            visitables.push(quote! {
                impl VisitableMut for #elem_type {
                    fn apply(&mut self, visitor: &mut impl VisitorMut) {
                        visitor.#fn_name(self);
                    }
                }
            });
        }

        // Built complete visitor implementation
        quote! {
            pub trait VisitorMut {
                #(#trait_functions)*
            }

            #(#helpers)*

            trait VisitableMut {
                fn apply(&mut self, visitor: &mut impl VisitorMut);
            }
            #(#visitables)*
        }
    }
}

/// Private struct for dissecting each option passed to a visitor type
struct Options {
    no_default: bool,
    has_helper: bool,
    helper_tmpl: Option<TokenStream>,
}

impl Options {
    fn new(options: &Punctuated<KeyValue, Token![,]>) -> Self {
        // Defaults
        let mut no_default = false;
        let mut has_helper = true;
        let mut helper_tmpl = None;

        // Loop over each option given
        for option in options.iter() {
            // "no_default" turns no_default on
            if option.key == Ident::new("no_default", Span::call_site()) {
                no_default = true;
                continue;
            }

            if option.key == Ident::new("helper_tmpl", Span::call_site()) {
                match &option.value {
                    TokenTree::Ident(ident) if ident == &Ident::new("false", Span::call_site()) => {
                        // "helper_tmpl = false" turns helper template off
                        has_helper = false;
                    }
                    TokenTree::Group(group) => {
                        // Custom helper template was given
                        helper_tmpl = Some(group.stream());
                    }
                    _ => continue,
                }
            }
        }

        Options {
            no_default,
            has_helper,
            helper_tmpl,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use despatma_test_helpers::reformat;
    use pretty_assertions::assert_eq;
    use syn::{parse_quote, parse_str};

    type Result = std::result::Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn parse() {
        let actual: VisitorFunction = parse_quote! {
            #[no_default]
            dyn Button
        };

        let mut expected = VisitorFunction {
            types: Punctuated::new(),
        };

        expected.types.push(parse_quote! {#[no_default] dyn Button});

        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_just_types() -> Result {
        let actual: VisitorFunction = parse_str("Button, dyn Text, Window")?;

        let mut expected = VisitorFunction {
            types: Punctuated::new(),
        };

        expected.types.push(parse_str("Button")?);
        expected.types.push(parse_str("dyn Text")?);
        expected.types.push(parse_str("Window")?);

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn parse_mixed() -> Result {
        let actual: VisitorFunction = parse_quote! {
            Button,

            #[tmpl = {trait T {};}]
            Text,

            dyn Window
        };

        let mut expected = VisitorFunction {
            types: Punctuated::new(),
        };

        expected.types.push(parse_str("Button")?);
        expected.types.push(parse_quote! {
            #[tmpl = {trait T {};}]
            Text
        });
        expected.types.push(parse_str("dyn Window")?);

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn expand() -> Result {
        let mut input = VisitorFunction {
            types: Punctuated::new(),
        };

        input.types.push(parse_quote! {
            #[helper_tmpl = false]
            Button
        });
        input.types.push(parse_quote! {
            #[no_default]
            dyn Text
        });
        input.types.push(parse_quote! {
           #[helper_tmpl = {
               visitor.visit_button(&window.button);
           }]
           Window
        });

        let actual = input.expand();
        let expected = quote! {
            pub trait Visitor{
                fn visit_button(&mut self, button: &Button) {
                    visit_button(self, button)
                }
                fn visit_text(&mut self, text: &dyn Text);
                fn visit_window(&mut self, window: &Window) {
                    visit_window(self, window)
                }
            }

            pub fn visit_text<V>(_visitor: &mut V, _text: &dyn Text)
            where
                V: Visitor + ?Sized,
            {
            }

            pub fn visit_window<V>(visitor: &mut V, window: &Window)
            where
                V: Visitor + ?Sized,
            {
               visitor.visit_button(&window.button);
            }

            trait Visitable {
                fn apply(&self, visitor: &mut impl Visitor);
            }
            impl Visitable for Button {
                fn apply(&self, visitor: &mut impl Visitor) {
                    visitor.visit_button(self);
                }
            }
            impl Visitable for dyn Text {
                fn apply(&self, visitor: &mut impl Visitor) {
                    visitor.visit_text(self);
                }
            }
            impl Visitable for Window {
                fn apply(&self, visitor: &mut impl Visitor) {
                    visitor.visit_window(self);
                }
            }
        };

        assert_eq!(
            reformat(&actual).lines().collect::<Vec<_>>(),
            reformat(&expected).lines().collect::<Vec<_>>()
        );

        Ok(())
    }
}
