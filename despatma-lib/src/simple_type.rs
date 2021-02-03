use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Ident, Token};

/// Holds a simple type that is optionally annotated as `dyn`.
///
/// The following is an example of its input stream:
/// ```text
/// dyn SomeType
/// ```
///
/// The `dyn` keyword is optional.
#[cfg_attr(any(test, feature = "extra-traits"), derive(Eq, PartialEq, Debug))]
pub struct SimpleType {
    pub dyn_token: Option<Token![dyn]>,
    pub ident: Ident,
}

/// Make SimpleType parsable from token stream
impl Parse for SimpleType {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(SimpleType {
            dyn_token: input.parse()?,
            ident: input.parse()?,
        })
    }
}

/// Turn SimpleType back into a token stream
impl ToTokens for SimpleType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.dyn_token.to_tokens(tokens);
        self.ident.to_tokens(tokens);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use quote::quote;
    use syn::parse_str;

    type Result = std::result::Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn parse() -> Result {
        let actual: SimpleType = parse_str("dyn Button")?;
        let expected = SimpleType {
            dyn_token: Some(Default::default()),
            ident: parse_str("Button")?,
        };

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn parse_without_dyn() -> Result {
        let actual: SimpleType = parse_str("Button")?;
        let expected = SimpleType {
            dyn_token: None,
            ident: parse_str("Button")?,
        };

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    #[should_panic(expected = "expected identifier")]
    fn missing_type() {
        parse_str::<SimpleType>("dyn").unwrap();
    }

    #[test]
    fn to_tokens() -> Result {
        let input = SimpleType {
            dyn_token: Some(Default::default()),
            ident: parse_str("Button")?,
        };
        let actual = quote! { #input };
        let expected: TokenStream = parse_str("dyn Button")?;

        assert_eq!(format!("{}", actual), format!("{}", expected));
        Ok(())
    }
}
