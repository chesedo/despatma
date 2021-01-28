use crate::key_value::KeyValue;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{bracketed, token, Token};

/// Holds an outer attribute filled with key-value options.
/// Streams in the following form will be parsed successfully:
/// ```text
/// #[key1 = value1, bool_key2, key3 = value]
/// ```
///
/// The value part of an option is optional.
/// Thus, `bool_key2` will have the value `default`.
#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
#[derive(Default)]
pub struct OptionsAttribute {
    pub pound_token: Token![#],
    pub bracket_token: token::Bracket,
    pub options: Punctuated<KeyValue, Token![,]>,
}

/// Make OptionsAttribute parsable from a token stream
impl Parse for OptionsAttribute {
    fn parse(input: ParseStream) -> Result<Self> {
        // Used to hold the stream inside square brackets
        let content;

        Ok(OptionsAttribute {
            pound_token: input.parse()?,
            bracket_token: bracketed!(content in input), // Use `syn` to extract the stream inside the square bracket group
            options: content.parse_terminated(KeyValue::parse)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use syn::parse_str;

    type Result = std::result::Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn parse() -> Result {
        let actual: OptionsAttribute =
            parse_str("#[tmpl = {trait To {};}, no_default, other = Top]")?;
        let mut expected = OptionsAttribute {
            pound_token: Default::default(),
            bracket_token: Default::default(),
            options: Punctuated::new(),
        };

        expected.options.push(parse_str("tmpl = {trait To {};}")?);
        expected.options.push(parse_str("no_default")?);
        expected.options.push(parse_str("other = Top")?);

        assert_eq!(actual, expected);
        Ok(())
    }
}
