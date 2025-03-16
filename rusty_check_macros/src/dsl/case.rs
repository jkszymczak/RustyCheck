use proc_macro2::TokenStream as TS;
use quote::quote;
use syn::{braced, parse::Parse,Token};

use super::{check::Check, given::Given, keywords as kw, traits::Code};

pub struct Case {
    kw: kw::case,
    ident: syn::Ident,
    given: Option<Given>,
    check: Check
}

impl Parse for Case {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let kw = input.parse::<kw::case>()?;
        let ident = input.parse::<syn::Ident>()?;
        let case;
        braced!(case in input);
        let given = if case.peek(kw::given) {Some(case.parse::<Given>()?)} else {None};
        let check = case.parse::<Check>()?;
        Ok(Case { kw, ident, given, check })
    }
}

impl Code for Case {
    fn get_code(&self) -> proc_macro2::TokenStream {
        let ident = self.ident.clone();
        let given = match &self.given {
            Some(given) => given.get_code(),
            None => quote! {},
        };
        let check = self.check.get_code();
        quote! {
            #[test]
            fn #ident() {
                #given
                #check;
            }
        }
    }
}
