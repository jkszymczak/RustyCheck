use super::case::Case;
use super::keywords as kw;
use super::traits::Code;
use proc_macro2::TokenStream as TS;
use quote::quote;
use syn::{braced, parse::Parse, Error, Token};
pub struct RustyCheck {
    cases: Vec<Case>,
}

impl Parse for RustyCheck {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut cases: Vec<Case> = Vec::new();
        while !input.is_empty() {
            if input.peek(kw::case) {
                cases.push(input.parse::<Case>()?);
            } else {
                return Err(input.error("Expected Case"));
            }
        }
        Ok(RustyCheck { cases })
    }
}

impl Code for RustyCheck {
    fn get_code(&self) -> proc_macro2::TokenStream {
        let cases = self.cases.iter().map(|case| case.get_code());
        quote! {
            #(#cases)*
        }
    }
}
