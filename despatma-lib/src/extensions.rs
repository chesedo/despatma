use syn::Ident;

/// Extension helper to turn part of a token stream into lowercase
pub trait ToLowercase {
    /// Create a lower case copy of this token
    fn to_lowercase(&self) -> Self;
}

/// Allow `Ident`s to be turned into lowercase
impl ToLowercase for Ident {
    fn to_lowercase(&self) -> Self {
        Ident::new(&format!("{}", self).to_lowercase(), self.span())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use syn::parse_quote;

    #[test]
    fn ident_to_lowercase() {
        let input: Ident = parse_quote! {MixedIdentifier};
        let actual = input.to_lowercase();
        let expected: Ident = parse_quote! {mixedidentifier};

        assert_eq!(actual, expected);
    }
}
