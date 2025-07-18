use std::marker::PhantomData;

use super::{super::super::traits::Code, expression::Expression, keywords as kw};
use proc_macro2::TokenStream as TS;
use quote::quote;
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
    let ident = input.parse::<syn::Ident>()?;
    input.parse::<Token![=]>()?;
    let exp = input.parse::<Expression>()?.get_code();
    let code: TS = quote! {
        #assignment_kw #ident = #exp;
    }
    .into();
    Ok(Assignment {
        kw: PhantomData,
        data: code,
    })
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
            parse_assignment(input, quote! {static mut}.into())
        } else {
            parse_assignment(input, quote! {static}.into())
        }
    }
}
impl Parse for Assignment<kw::constants> {
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
impl<K: Parse> Code for DeclarationBlock<K> {
    fn get_code(&self) -> proc_macro2::TokenStream {
        let assignments = self.assignments.iter().map(|a| a.data.clone());
        quote! {
            #(#assignments)*
        }
    }
}
