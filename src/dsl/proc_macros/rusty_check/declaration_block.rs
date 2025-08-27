use std::marker::PhantomData;

use super::keywords as kw;
use proc_macro2::TokenStream as TS;

use quote::{quote, ToTokens};

use syn::{braced, parse::Parse, Expr, Token};

/// Represents a block of declarations, parameterized by a keyword type `K`.
///
/// # Type Parameters
/// - `K`: A type that implements the `Parse` trait, representing the keyword for the block.
///
/// # Fields
/// - `kw`: The keyword associated with the declaration block.
/// - `assignments`: A list of assignments within the block.
///
/// Used for representing grammar from this diagram:
///
#[derive(Clone)]
pub struct DeclarationBlock<K: Parse> {
    pub kw: K,
    assignments: Vec<Assignment<K>>,
}

/// Represents an assignment within a declaration block, parameterized by a keyword type `K`.
///
/// # Type Parameters
/// - `K`: A type that implements the `Parse` trait, representing the keyword for the assignment.
///
/// # Fields
/// - `kw`: A phantom type to associate the assignment with the keyword type `K`.
/// - `data`: The token stream representing the assignment.
#[derive(Clone)]
pub struct Assignment<K: Parse> {
    kw: PhantomData<K>,
    data: TS,
}

/// Parses an assignment from the input stream.
///
/// # Parameters
/// - `input`: The parse stream to read from.
/// - `assignment_kw`: The token stream representing the keyword for the assignment.
///
/// # Returns
/// A parsed `Assignment` instance.
///
/// # Errors
/// Returns a `syn::Error` if the input cannot be parsed as an assignment.
fn parse_assignment<K: Parse>(
    input: syn::parse::ParseStream,
    assignment_kw: TS,
) -> syn::Result<Assignment<K>> {
    let ident = if input.peek2(Token![:]) {
        input.parse::<syn::PatType>()?.to_token_stream()
    } else {
        input.parse::<syn::Ident>()?.to_token_stream()
    };
    input.parse::<Token![=]>()?;
    let exp = input.parse::<Expr>()?;
    let code: TS = quote! {
        #assignment_kw #ident = #exp;
    }
    .into();
    Ok(Assignment {
        kw: PhantomData,
        data: code,
    })
}

impl<K: Parse> ToTokens for Assignment<K> {
    /// Converts the `Assignment` into tokens and appends them to the provided token stream.
    ///
    /// # Parameters
    /// - `tokens`: The token stream to append to.
    fn to_tokens(&self, tokens: &mut TS) {
        tokens.extend(self.data.clone().into_iter());
    }
}

impl Parse for Assignment<kw::given> {
    /// Parses an `Assignment` with the `given` keyword from the input stream.
    ///
    /// # Parameters
    /// - `input`: The parse stream to read from.
    ///
    /// # Returns
    /// A parsed `Assignment<kw::given>` instance.
    ///
    /// # Errors
    /// Returns a `syn::Error` if the input cannot be parsed as an assignment.
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Token![mut]) {
            _ = input.parse::<Token![mut]>();
            parse_assignment(input, quote! {let mut}.into())
        } else {
            parse_assignment(input, quote! {let}.into())
        }
    }
}

impl Parse for Assignment<kw::vars> {
    /// Parses an `Assignment` with the `vars` keyword from the input stream.
    ///
    /// # Parameters
    /// - `input`: The parse stream to read from.
    ///
    /// # Returns
    /// A parsed `Assignment<kw::vars>` instance.
    ///
    /// # Errors
    /// Returns a `syn::Error` if the input cannot be parsed as an assignment.
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Token![mut]) {
            _ = input.parse::<Token![mut]>();

            parse_assignment(input, quote! {static mut}.into())
        } else {
            parse_assignment(input, quote! {static}.into())
        }
    }
}

impl Parse for Assignment<kw::consts> {
    /// Parses an `Assignment` with the `consts` keyword from the input stream.
    ///
    /// # Parameters
    /// - `input`: The parse stream to read from.
    ///
    /// # Returns
    /// A parsed `Assignment<kw::consts>` instance.
    ///
    /// # Errors
    /// Returns a `syn::Error` if the input cannot be parsed as an assignment.
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        parse_assignment(input, quote! {const}.into())
    }
}

impl<K: Parse> Parse for DeclarationBlock<K>
where
    Assignment<K>: Parse,
{
    /// Parses a `DeclarationBlock` from the input stream.
    ///
    /// # Parameters
    /// - `input`: The parse stream to read from.
    ///
    /// # Returns
    /// A parsed `DeclarationBlock<K>` instance.
    ///
    /// # Errors
    /// Returns a `syn::Error` if the input cannot be parsed as a declaration block.
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let kw = input.parse::<K>()?;
        let assignments;
        braced!(assignments in input);
        let assignments = assignments.parse_terminated(Assignment::parse, Token![,])?;
        let parsed_assignments: Vec<Assignment<K>> = assignments.into_iter().collect();
        Ok(DeclarationBlock {
            kw: kw,
            assignments: parsed_assignments,
        })
    }
}

impl<K: Parse> ToTokens for DeclarationBlock<K> {
    /// Converts the `DeclarationBlock` into tokens and appends them to the provided token stream.
    ///
    /// # Parameters
    /// - `tokens`: The token stream to append to.
    fn to_tokens(&self, tokens: &mut TS) {
        let assignments = self.assignments.iter().map(|a| a.data.clone());
        let code = quote! {
            #(#assignments)*
        };
        tokens.extend(code.into_iter());
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::{parse, parse2, parse_quote, parse_str, Token};

    #[test]
    fn test_parse_assignment_given() {
        let input: TokenStream = quote! {
            x = 42
        };
        let parsed: Assignment<kw::given> = parse2(input).unwrap();
        assert_eq!(
            parsed.data.to_string(),
            parse_str::<TS>("let x = 42;").unwrap().to_string()
        );
    }

    #[test]
    fn test_parse_assignment_given_mut() {
        let input: TokenStream = quote! {
            mut y = 100
        };
        let parsed: Assignment<kw::given> = parse2(input).unwrap();
        assert_eq!(
            parsed.data.to_string(),
            parse_str::<TS>("let mut y = 100;").unwrap().to_string()
        );
    }

    #[test]
    fn test_parse_assignment_vars() {
        let input: TokenStream = quote! {
            z = 50
        };
        let parsed: Assignment<kw::vars> = parse2(input).unwrap();
        assert_eq!(parsed.data.to_string(), "static z = 50 ;");
    }

    #[test]
    fn test_parse_assignment_vars_mut() {
        let input: TokenStream = quote! {
            mut w = 75
        };
        let parsed: Assignment<kw::vars> = parse2(input).unwrap();
        assert_eq!(parsed.data.to_string(), "static mut w = 75 ;");
    }

    #[test]
    fn test_parse_assignment_consts() {
        let input: TokenStream = quote! {
            c = 10
        };
        let parsed: Assignment<kw::consts> = parse2(input).unwrap();
        assert_eq!(parsed.data.to_string(), "const c = 10 ;");
    }

    #[test]
    fn test_parse_declaration_block_given() {
        let input: TokenStream = quote! {
            given { x = 42, y = 100 }
        };
        let parsed: DeclarationBlock<kw::given> = parse2(input).unwrap();
        assert_eq!(parsed.assignments.len(), 2);
        assert_eq!(parsed.assignments[0].data.to_string(), "let x = 42 ;");
        assert_eq!(parsed.assignments[1].data.to_string(), "let y = 100 ;");
    }

    #[test]
    fn test_parse_declaration_block_vars() {
        let input: TokenStream = quote! {
            vars { z = 50, w = 75 }
        };
        let parsed: DeclarationBlock<kw::vars> = parse2(input).unwrap();
        assert_eq!(parsed.assignments.len(), 2);
        assert_eq!(parsed.assignments[0].data.to_string(), "static z = 50 ;");
        assert_eq!(parsed.assignments[1].data.to_string(), "static w = 75 ;");
    }

    #[test]
    fn test_parse_declaration_block_consts() {
        let input: TokenStream = quote! {
            consts { c = 10 }
        };
        let parsed: DeclarationBlock<kw::consts> = parse2(input).unwrap();
        assert_eq!(parsed.assignments.len(), 1);
        assert_eq!(parsed.assignments[0].data.to_string(), "const c = 10 ;");
    }

    #[test]
    fn test_to_tokens_declaration_block() {
        let block: DeclarationBlock<kw::given> = parse_quote! {
            given { x = 42, y = 100 }
        };
        let tokens = block.to_token_stream();

        assert_eq!(tokens.to_string(), "let x = 42 ; let y = 100 ;");
    }
}
