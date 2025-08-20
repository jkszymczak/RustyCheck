use super::{
    super::helpers::ToComment,
    conditions::Conditions,
    configure::{CommentType, Config},
    keywords as kw,
};
use proc_macro2::TokenStream as TS;
use quote::{quote, ToTokens};
use syn::{braced, parse::Parse};

/// Represents a `Check` block in the RustyCheck DSL.
///
/// A `Check` block is used to define assertions in the DSL. It contains:
/// - A `keyword`: The `check` keyword.
/// - `conditions`: The conditions to be asserted.
/// - `comment`: A string representation of the conditions for debugging or documentation purposes.
///
/// represents grammar from this diagram:
///
#[doc = include_str!("../../../../../grammar/case/check/check.svg")]
#[derive(Clone)]
pub struct Check {
    keyword: kw::check,
    conditions: Conditions,
    comment: String,
    comment_type: CommentType,
    test_unstable: bool,
}

impl Check {
    fn new(kw: kw::check, comment: String, conditions: Conditions) -> Check {
        Check {
            keyword: kw,
            comment,
            conditions,
            comment_type: CommentType::default(),
            test_unstable: false,
        }
    }
    pub fn set_options(self, config: &Config) -> Check {
        Check {
            comment_type: config.get_comment_type().unwrap_or(self.comment_type),
            test_unstable: config.get_unstabe_test().unwrap_or(self.test_unstable),
            ..self
        }
    }
}

impl Parse for Check {
    /// Parses a `Check` block from the input stream.
    ///
    /// # Parameters
    /// - `input`: The parse stream to read from.
    ///
    /// # Returns
    /// A parsed `Check` instance containing the keyword, conditions, and a comment.
    ///
    /// # Errors
    /// Returns a `syn::Error` if the input cannot be parsed as a valid `Check` block.
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let kw = input.parse::<kw::check>()?;
        let conditions;
        braced!(conditions in input);
        let comment = conditions.to_string();
        let conditions = conditions.parse::<Conditions>()?;
        Ok(Check::new(kw, comment, conditions))
    }
}

impl ToTokens for Check {
    /// Converts the `Check` block into tokens for code generation.
    ///
    /// This implementation generates an `assert!` statement with the conditions
    /// and a comment for debugging purposes.
    ///
    /// # Parameters
    /// - `tokens`: The token stream to append the generated code to.
    fn to_tokens(&self, tokens: &mut TS) {
        let conditions = &self.conditions;
        let comment = &self.conditions.to_comment(self.comment_type);
        if self.test_unstable {
            tokens.extend(quote! {
                 if !#conditions {
                     //TODO remember to add condition for unstable tests to run cargo test with -- --nocapture or --show-output
                     eprintln!("Unstable test failed, {}",#comment);
                 }
            });
        } else {
            tokens.extend(quote! {assert!(#conditions,#comment);});
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::parse_str;

    use super::*;
    #[test]
    fn test_check() {
        let check = parse_str::<Check>("check { a equal 10 }")
            .unwrap()
            .to_token_stream()
            .to_string();
        let result = parse_str::<TS>("assert!((a==10),\"a equal 10 where, a={:?}\" ,a );")
            .unwrap()
            .to_string();
        assert_eq!(check, result);
    }
}
