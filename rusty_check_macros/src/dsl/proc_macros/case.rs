use proc_macro2::TokenStream as TS;
use quote::quote;
use syn::{braced, parse::Parse, Token};

use super::{super::traits::Code, check::Check, compute::Compute, given::Given, keywords as kw};

pub struct Case {
    kw: kw::case,
    ident: syn::Ident,
    given: Option<Given>,
    compute: Option<Compute>,
    check: Check,
}

impl Parse for Case {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let kw = input.parse::<kw::case>()?;
        let ident = input.parse::<syn::Ident>()?;
        let case;
        braced!(case in input);
        let given = if case.peek(kw::given) {
            Some(case.parse::<Given>()?)
        } else {
            None
        };
        let compute = if case.peek(Token![do]) {
            Some(case.parse::<Compute>()?)
        } else {
            None
        };
        let check = case.parse::<Check>()?;
        Ok(Case {
            kw,
            ident,
            given,
            compute,
            check,
        })
    }
}

impl Code for Case {
    fn get_code(&self) -> proc_macro2::TokenStream {
        let ident = self.ident.clone();
        let given = match &self.given {
            Some(given) => given.get_code(),
            None => quote! {},
        };
        let compute = match &self.compute {
            Some(compute) => compute.get_code(),
            None => quote! {},
        };
        let check = self.check.get_code();
        quote! {
            #[test]
            fn #ident() {
                #given
                #compute
                #check;
            }
        }
    }
}
