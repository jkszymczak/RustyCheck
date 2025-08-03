use std::marker::PhantomData;

use super::{expression::Expression, keywords as kw};
use proc_macro2::TokenStream as TS;

use quote::{quote, ToTokens};

use syn::{braced, parse::Parse, Token};

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
#[doc = include_str!("../../../../../grammar/assignment.svg")]
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
    let exp = input.parse::<Expression>()?;
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
