use proc_macro2::TokenStream as TS;
use quote::{quote, ToTokens};
use syn::{braced, parse::Parse, Token};

/// Represents a `Compute` block in the RustyCheck DSL.
///
/// A `Compute` block is used to define a section of Rust code that will be executed
/// as part of the DSL. It contains:
/// - `keyword`: The `do` keyword that introduces the block.
/// - `rust_code`: The Rust code enclosed within the block.
///
/// represents grammar from this diagram:
///
#[doc = include_str!("../../../../../grammar/case/compute.svg")]
#[derive(Clone)]
pub struct Compute {
    keyword: Token![do],
    rust_code: TS,
}

impl Parse for Compute {
    /// Parses a `Compute` block from the input stream.
    ///
    /// # Parameters
    /// - `input`: The parse stream to read from.
    ///
    /// # Returns
    /// A parsed `Compute` instance containing the `do` keyword and the Rust code.
    ///
    /// # Errors
    /// Returns a `syn::Error` if the input cannot be parsed as a valid `Compute` block.
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let kw = input.parse::<Token![do]>()?;
        let rust_code;
        braced!(rust_code in input);
        Ok(Compute {
            keyword: kw,
            rust_code: rust_code.parse()?,
        })
    }
}

impl ToTokens for Compute {
    /// Converts the `Compute` block into tokens for code generation.
    ///
    /// This implementation generates the Rust code contained within the block.
    ///
    /// # Parameters
    /// - `tokens`: The token stream to append the generated code to.
    fn to_tokens(&self, tokens: &mut TS) {
        let code = self.rust_code.clone();
        tokens.extend(quote! {#code});
    }
}
