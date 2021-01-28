use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{parse_quote, ItemTrait, Token, Type, TypeParamBound};

/// Holds the tokens for the attributes passed to an AbstractFactory attribute macro
/// Expects input in the following format
/// ```text
/// some_factory_method_trait, type_1, type_2, ... , type_n
/// ```
///
/// `some_factory_method_trait` needs to be created by the client.
/// It should have one generic type.
/// Every `type_1` ... `type_n` will be filled into this generic type.
#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
pub struct AbstractFactoryAttribute {
    factory_trait: Type,
    sep: Token![,],
    types: Punctuated<Type, Token![,]>,
}

/// Make AbstractFactoryAttribute parsable
impl Parse for AbstractFactoryAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(AbstractFactoryAttribute {
            factory_trait: input.parse()?,
            sep: input.parse()?,
            types: input.parse_terminated(Type::parse)?,
        })
    }
}

impl AbstractFactoryAttribute {
    /// Add factory super traits to an `ItemTrait` to turn the `ItemTrait` into an Abstract Factory
    pub fn expand(&self, input_trait: &mut ItemTrait) -> TokenStream {
        // Build all the super traits
        let factory_traits: Punctuated<TypeParamBound, Token![+]> = {
            let types = self.types.iter();
            let factory_name = &self.factory_trait;

            parse_quote! {
                #(#factory_name<#types>)+*
            }
        };

        // Append extra factory super traits
        input_trait.supertraits.extend(factory_traits);

        quote! {
            #input_trait
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use despatma_test_helpers::reformat;
    use syn::parse_str;

    type Result = std::result::Result<(), Box<dyn std::error::Error>>;

    mod abstract_factory {
        use super::*;
        use pretty_assertions::assert_eq;

        #[test]
        fn parse() -> Result {
            let actual: AbstractFactoryAttribute = parse_str("Factory, u32, i64")?;
            let mut expected_types = Punctuated::new();

            expected_types.push(parse_str("u32")?);
            expected_types.push(parse_str("i64")?);

            assert_eq!(
                actual,
                AbstractFactoryAttribute {
                    factory_trait: parse_str("Factory")?,
                    sep: Default::default(),
                    types: expected_types,
                }
            );

            Ok(())
        }

        #[test]
        #[should_panic(expected = "expected `,`")]
        fn missing_types() {
            parse_str::<AbstractFactoryAttribute>("Factory").unwrap();
        }

        #[test]
        fn expand() -> Result {
            let mut t = parse_str::<ItemTrait>("pub trait Abstraction<T>: Display + Extend<T> {}")?;
            let mut input_types = Punctuated::new();

            input_types.push(parse_str("u32")?);
            input_types.push(parse_str("i64")?);

            let actual = &AbstractFactoryAttribute {
                factory_trait: parse_str("Factory")?,
                sep: Default::default(),
                types: input_types,
            }
            .expand(&mut t);

            assert_eq!(
                reformat(&actual),
                "pub trait Abstraction<T>: Display + Extend<T> + Factory<u32> + Factory<i64> {}\n"
            );

            Ok(())
        }
    }
}
