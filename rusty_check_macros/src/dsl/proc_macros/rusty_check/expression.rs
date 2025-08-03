use super::keywords as kw;
use proc_macro2::TokenStream as TS;
use quote::{quote, ToTokens};
use syn::parse::Parse;

/// Represents a block of Rust code in the RustyCheck DSL.
///
/// A `RustBlock` is introduced by the `rust` keyword and contains:
/// - `keyword`: The `rust` keyword.
/// - `data`: The token stream representing the Rust code within the block.
pub struct RustBlock {
    keyword: kw::rust,
    data: TS,
}

/// Represents an expression in the RustyCheck DSL.
///
/// An `Expression` can be one of the following:
/// - `RustExpression`: A single Rust expression.
/// - `RustBlock`: A block of Rust code introduced by the `rust` keyword.
pub enum Expression {
    RustExpression(TS),
    RustBlock(RustBlock),
}

impl Parse for Expression {
    /// Parses an `Expression` from the input stream.
    ///
    /// # Parameters
    /// - `input`: The parse stream to read from.
    ///
    /// # Returns
    /// A parsed `Expression` instance, which can be either a `RustExpression` or a `RustBlock`.
    ///
    /// # Errors
    /// Returns a `syn::Error` if the input cannot be parsed as a valid `Expression`.
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(kw::rust) {
            let kw = input.parse::<kw::rust>()?;
            let code = input.parse::<syn::ExprBlock>()?;
            let code: TS = quote! { #code }.into();
            let block = RustBlock {
                keyword: kw,
                data: code,
            };
            Ok(Expression::RustBlock(block))
        } else {
            let exp = input.parse::<syn::Expr>()?;
            let exp: TS = quote! {#exp};
            Ok(Expression::RustExpression(exp))
        }
    }
}

impl ToTokens for Expression {
    /// Converts an `Expression` into tokens for code generation.
    ///
    /// This implementation generates the Rust code for the expression, whether it is
    /// a single `RustExpression` or a `RustBlock`.
    ///
    /// # Parameters
    /// - `tokens`: The token stream to append the generated code to.
    fn to_tokens(&self, tokens: &mut TS) {
        let code = match self {
            Self::RustExpression(code) => code.clone(),
            Self::RustBlock(block) => block.data.clone(),
        };
        tokens.extend(code);
    }
}
