use super::{super::traits::Code, expression::Expression, keywords as kw};
use proc_macro2::TokenStream as TS;
use quote::quote;
use syn::{braced, parse::Parse, Token};

pub struct Given {
    keyword: kw::given,
    assignments: Vec<Assignment>,
}
// struct Assignment {
//     data: TS,
// }

pub enum Assignment {
    Mutable { data: TS },
    Immutable { data: TS },
}

fn parse_immutable(input: syn::parse::ParseStream) -> syn::Result<Assignment> {
    let ident = input.parse::<syn::Ident>()?;
    input.parse::<Token![=]>()?;
    let exp = input.parse::<Expression>()?.get_code();
    let code: TS = quote! {
        let #ident = #exp;
    }
    .into();
    Ok(Assignment::Immutable { data: code })
}

fn parse_mutable(input: syn::parse::ParseStream) -> syn::Result<Assignment> {
    _ = input.parse::<Token![mut]>()?;
    let ident = input.parse::<syn::Ident>()?;
    input.parse::<Token![=]>()?;
    let exp = input.parse::<Expression>()?.get_code();
    let code: TS = quote! {
        let mut #ident = #exp;
    }
    .into();
    Ok(Assignment::Mutable { data: code })
}
impl Parse for Assignment {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Token![mut]) {
            parse_mutable(input)
        } else {
            parse_immutable(input)
        }
    }
}

impl Parse for Given {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let kw = input.parse::<kw::given>()?;
        let assignments;
        braced!(assignments in input);
        let assignments = assignments.parse_terminated(Assignment::parse, Token![,])?;
        let parsed_assignments: Vec<Assignment> = assignments.into_iter().collect();
        Ok(Given {
            keyword: kw,
            assignments: parsed_assignments,
        })
    }
}
impl Code for Given {
    fn get_code(&self) -> proc_macro2::TokenStream {
        let assignments = self.assignments.iter().map(|a| match a {
            Assignment::Mutable { data } => data.clone(),
            Assignment::Immutable { data } => data.clone(),
        });
        quote! {
            #(#assignments)*
        }
    }
}
