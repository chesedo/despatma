use proc_macro2::TokenStream;
use quote::ToTokens;
use std::collections::HashMap;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Token, Type};
use tokenstream2_tmpl::{interpolate, Interpolate};

/// Type that holds an abstract type and how it will map to a concrete type.
///
/// An acceptable stream will have the following form:
/// ```text
/// trait => concrete
/// ```
#[cfg_attr(any(test, feature = "extra-traits"), derive(Eq, PartialEq, Debug))]
pub struct TraitSpecifier {
    pub abstract_trait: Type,
    pub arrow_token: Token![=>],
    pub concrete: Type,
}

/// Make TraitSpecifier parsable from a token stream
impl Parse for TraitSpecifier {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(TraitSpecifier {
            abstract_trait: input.parse()?,
            arrow_token: input.parse()?,
            concrete: input.parse()?,
        })
    }
}

/// Make TraitSpecifier interpolatible
impl Interpolate for TraitSpecifier {
    fn interpolate(&self, stream: TokenStream) -> TokenStream {
        let mut replacements: HashMap<_, &dyn ToTokens> = HashMap::new();

        // Replace each "TRAIT" with the absract trait
        replacements.insert("TRAIT", &self.abstract_trait);

        // Replace each "CONCRETE" with the concrete type
        replacements.insert("CONCRETE", &self.concrete);

        interpolate(stream, &replacements)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use despatma_test_helpers::reformat;
    use pretty_assertions::assert_eq;
    use quote::quote;
    use syn::parse_str;

    type Result = std::result::Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn parse() -> Result {
        let actual: TraitSpecifier = parse_str("abstract_trait => concrete")?;
        let expected = TraitSpecifier {
            abstract_trait: parse_str("abstract_trait")?,
            arrow_token: Default::default(),
            concrete: parse_str("concrete")?,
        };

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    #[should_panic(expected = "expected one of")]
    fn missing_trait() {
        parse_str::<TraitSpecifier>("=> concrete").unwrap();
    }

    #[test]
    #[should_panic(expected = "expected `=>`")]
    fn missing_arrow_joiner() {
        parse_str::<TraitSpecifier>("IButton -> RoundButton").unwrap();
    }

    #[test]
    #[should_panic(expected = "unexpected end of input")]
    fn missing_concrete() {
        parse_str::<TraitSpecifier>("abstract_trait => ").unwrap();
    }

    #[test]
    fn interpolate() -> Result {
        let input = quote! {
            impl Factory<TRAIT> for Gnome {
                fn create(&self) -> CONCRETE {
                    CONCRETE{}
                }
            }
        };
        let expected = quote! {
            impl Factory<abstract_trait> for Gnome {
                fn create(&self) -> concrete {
                    concrete{}
                }
            }
        };
        let specifier = TraitSpecifier {
            abstract_trait: parse_str("abstract_trait")?,
            arrow_token: Default::default(),
            concrete: parse_str("concrete")?,
        };

        assert_eq!(reformat(&specifier.interpolate(input)), reformat(&expected));

        Ok(())
    }
}
