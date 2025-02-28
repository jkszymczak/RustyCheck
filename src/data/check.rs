// FIXME Need to rewrite this whole module :c
use crate::data::{keywords as kw, traits::Code};
use syn::{
    braced,
    parse::{Parse, ParseBuffer},
    Expr,
};

use quote::quote;

pub struct Check {
    keyword: kw::check,
    data: proc_macro2::TokenStream,
}


//TODO think about data structure before functions

// What for conditions?
//     a is equal/less than b
//     as are equal /less than bs
//     any as are equal b
//     a is equal any bs

struct Condition {
    is_negated: bool,
    left_is_many: bool,
    left: proc_macro2::TokenStream,
    symbol: proc_macro2::TokenStream,
    right_is_many: bool,
    right: proc_macro2::TokenStream,
}

fn parse_symbol(input: syn::parse::ParseStream) -> syn::Result<proc_macro2::TokenStream> {
    input.parse::<kw::equal>()?;
    Ok(quote! {==}.into())
}

impl Parse for Condition {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let left = input.parse::<syn::Expr>()?;
        let left_is_many = if input.peek(kw::are) {
            input.parse::<kw::are>()?;
            true
        } else {
            input.parse::<kw::is>()?;
            false
        };
        let is_negated = if input.peek(kw::not) {
            input.parse::<kw::not>()?;
            true
        } else {
            false
        };
        let symbol = parse_symbol(&input)?;
        let right = input.parse::<syn::Expr>()?;
        let left = quote! {#left}.into();
        let right = quote! {#right}.into();
        Ok(Condition {
            is_negated: is_negated,
            left_is_many: left_is_many,
            left: left,
            symbol: symbol,
            right_is_many: false,
            right: right,
        })
    }
}

impl Code for Condition {
    fn get_code(&self) -> proc_macro2::TokenStream {
        let left = self.left.clone();
        let right = self.right.clone();
        let symbol = self.symbol.clone();
        let condition: proc_macro2::TokenStream = quote! { #left #symbol #right }.into();
        let if_negated: proc_macro2::TokenStream = if self.is_negated {
            quote! { !(#condition) }.into()
        } else {
            condition
        };
        quote! { assert!(#if_negated)}.into()
    }
}

fn create_condition(input: ParseBuffer<'_>) -> syn::Result<proc_macro2::TokenStream> {
    let left: Expr = input.parse::<syn::Expr>()?;
    input.parse::<kw::is>()?;
    input.parse::<kw::equal>()?;
    let symbol: proc_macro2::TokenStream = quote! { == }.into();
    let right = input.parse::<syn::Expr>()?;
    Ok(quote! {assert!( #left #symbol #right) }.into())
}

impl Parse for Check {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let kw = input.parse::<kw::check>()?;
        let conditionals;
        braced!(conditionals in input);
        let condition = conditionals.parse::<Condition>()?.get_code();
        Ok(Check {
            keyword: kw,
            data: condition,
        })
    }
}

impl Code for Check {
    fn get_code(&self) -> proc_macro2::TokenStream {
        self.data.clone()
    }
}
