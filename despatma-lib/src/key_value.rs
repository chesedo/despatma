use proc_macro2::TokenTree;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_str, Ident, Token};

/// Holds a single key value attribute, with the value being optional.
/// Streams in the following form will be parsed:
/// ```text
/// key = value
/// ```
///
/// The `value` is optional.
/// Thus, the following is also valid.
/// ```text
/// key
/// ```
#[cfg_attr(any(test, feature = "extra-traits"), derive(Debug))]
pub struct KeyValue {
    pub key: Ident,
    pub equal_token: Token![=],
    pub value: TokenTree,
}

/// Make KeyValue parsable from a token stream
impl Parse for KeyValue {
    fn parse(input: ParseStream) -> Result<Self> {
        let key = input.parse()?;

        // Stop if optional value is not given
        if input.is_empty() || input.peek(Token![,]) {
            return Ok(KeyValue {
                key,
                equal_token: Default::default(),
                value: parse_str("default")?,
            });
        }

        // Parse with value
        Ok(KeyValue {
            key,
            equal_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

// Just for testing
#[cfg(any(test, feature = "extra-traits"))]
impl PartialEq for KeyValue {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && format!("{}", self.value) == format!("{}", other.value)
    }
}
#[cfg(any(test, feature = "extra-traits"))]
impl Eq for KeyValue {}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use syn::parse_str;

    type Result = std::result::Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn parse() -> Result {
        let actual: KeyValue = parse_str("some_key = \"value\"")?;
        let expected = KeyValue {
            key: parse_str("some_key")?,
            equal_token: Default::default(),
            value: parse_str("\"value\"")?,
        };

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn parse_missing_value() -> Result {
        let actual: KeyValue = parse_str("bool_key")?;
        let expected = KeyValue {
            key: parse_str("bool_key")?,
            equal_token: Default::default(),
            value: parse_str("default")?,
        };

        assert_eq!(actual, expected);
        Ok(())
    }

    #[test]
    fn parse_attribute_item_complex_stream() -> Result {
        let actual: KeyValue = parse_str("tmpl = {trait To {};}")?;
        let expected = KeyValue {
            key: parse_str("tmpl")?,
            equal_token: Default::default(),
            value: parse_str("{trait To {};}")?,
        };

        assert_eq!(actual, expected);
        Ok(())
    }

    // Test extra input after a value stream is ignored
    #[test]
    #[should_panic(expected = "expected token")]
    fn parse_attribute_item_complex_stream_extra() {
        parse_str::<KeyValue>("tmpl = {trait To {};}, key").unwrap();
    }

    #[test]
    #[should_panic(expected = "expected identifier")]
    fn missing_key() {
        parse_str::<KeyValue>("= true").unwrap();
    }

    #[test]
    #[should_panic(expected = "expected `=`")]
    fn missing_equal_sign() {
        parse_str::<KeyValue>("key  value").unwrap();
    }

    #[test]
    #[should_panic(expected = "expected token tree")]
    fn missing_value() {
        parse_str::<KeyValue>("key = ").unwrap();
    }
}
