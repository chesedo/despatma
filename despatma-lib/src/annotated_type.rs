use crate::options_attribute::OptionsAttribute;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Token, Type};

/// Holds a type that is optionally annotated with key-value options.
///
/// An acceptable stream will have the following form:
/// ```text
/// #[option1 = value1, option2 = value2]
/// SomeType
/// ```
///
/// The outer attribute (hash part) is optional.
/// `SomeType` will be parsed to `T`.
#[cfg_attr(any(test, feature = "extra-traits"), derive(Eq, PartialEq, Debug))]
pub struct AnnotatedType<T = Type> {
    pub attrs: OptionsAttribute,
    pub inner_type: T,
}

/// Make AnnotatedType parsable from token stream
impl<T: Parse> Parse for AnnotatedType<T> {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse attribute options if the next token is a hash
        if input.peek(Token![#]) {
            return Ok(AnnotatedType {
                attrs: input.parse()?,
                inner_type: input.parse()?,
            });
        };

        // Parse without attribute options
        Ok(AnnotatedType {
            attrs: Default::default(),
            inner_type: input.parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use syn::{parse_quote, parse_str, TypeTraitObject};

    type Result = std::result::Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn parse() -> Result {
        let actual: AnnotatedType = parse_quote! {
            #[no_default]
            i32
        };
        let expected = AnnotatedType {
            attrs: parse_str("#[no_default]")?,
            inner_type: parse_str("i32")?,
        };

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn parse_simple_type() -> Result {
        let actual: AnnotatedType = parse_quote! {
            Button
        };
        let expected = AnnotatedType {
            attrs: Default::default(),
            inner_type: parse_str("Button")?,
        };

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn parse_trait_bounds() -> Result {
        let actual: AnnotatedType<TypeTraitObject> = parse_quote! {
            #[no_default]
            dyn Button
        };
        let expected = AnnotatedType::<TypeTraitObject> {
            attrs: parse_str("#[no_default]")?,
            inner_type: parse_str("dyn Button")?,
        };

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    #[should_panic(expected = "unexpected end of input")]
    fn missing_type() {
        parse_str::<AnnotatedType>("#[no_default]").unwrap();
    }
}
