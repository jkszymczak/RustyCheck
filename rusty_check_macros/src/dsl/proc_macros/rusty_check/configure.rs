use super::keywords as kw;
use proc_macro2::{TokenStream as TS, TokenTree};
use quote::{quote, ToTokens};
use syn::{parse::Parse, token::Brace, Ident, Token};

/// Represents a configuration block in the RustyCheck DSL.
///
/// A `Config` block is used to define configuration options for a test case.
/// It contains:
/// - `keyword`: The `cfg` keyword that introduces the block.
/// - `elements`: The token stream representing the configuration values.
///
/// represents grammar from this diagram:
///
#[doc = include_str!("../../../../../grammar/global/cfg.svg")]
pub struct Config {
    keyword: kw::cfg,
    pub elements: TS,
}

impl ToTokens for Config {
    /// Converts the `Config` block into tokens for code generation.
    ///
    /// This implementation generates the Rust code contained within the configuration block.
    ///
    /// # Parameters
    /// - `tokens`: The token stream to append the generated code to.
    fn to_tokens(&self, tokens: &mut TS) {
        let code = self.elements.clone();
        tokens.extend(quote! {
            #code
        });
    }
}

impl Parse for Config {
    /// Parses a `Config` block from the input stream.
    ///
    /// # Parameters
    /// - `input`: The parse stream to read from.
    ///
    /// # Returns
    /// A parsed `Config` instance containing the `cfg` keyword and the configuration values.
    ///
    /// # Errors
    /// Returns a `syn::Error` if the input cannot be parsed as a valid `Config` block.
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let kw = input.parse::<kw::cfg>()?;
        _ = input.parse::<Token![=]>()?;
        let mut value_tokens = TS::new();
        while !input.is_empty() {
            let fork = input.fork();
            if fork.peek(Ident) && fork.peek2(Brace) {
                break; // found start of next statement
            }
            let tt: TokenTree = input.parse()?; // consume one token
            value_tokens.extend(std::iter::once(tt));
        }
        Ok(Config {
            keyword: kw,
            elements: value_tokens,
        })
    }
}
