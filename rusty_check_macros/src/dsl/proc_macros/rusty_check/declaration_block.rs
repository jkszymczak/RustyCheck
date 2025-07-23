use std::marker::PhantomData;

use super::{expression::Expression, keywords as kw};
use proc_macro2::TokenStream as TS;

use quote::{quote, ToTokens};

use syn::{braced, parse::Parse, Token};

pub struct DeclarationBlock<K: Parse> {
    pub kw: K,
    assignments: Vec<Assignment<K>>,
}

pub struct Assignment<K: Parse> {
    kw: PhantomData<K>,
    data: TS,
}

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
    fn to_tokens(&self, tokens: &mut TS) {
        tokens.extend(self.data.clone().into_iter());
    }
}

impl Parse for Assignment<kw::given> {
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
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        parse_assignment(input, quote! {const}.into())
    }
}
impl<K: Parse> Parse for DeclarationBlock<K>
where
    Assignment<K>: Parse,
{
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
    fn to_tokens(&self, tokens: &mut TS) {
        let assignments = self.assignments.iter().map(|a| a.data.clone());
        let code = quote! {
            #(#assignments)*
        };
        tokens.extend(code.into_iter());
    }
}
