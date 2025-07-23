use proc_macro2::TokenStream as TS;
use quote::{quote, ToTokens};
use syn::{braced, parse::Parse, Token};

use super::{
    check::Check, compute::Compute, configure::Config, declaration_block::DeclarationBlock,
    keywords as kw,
};

type Given = DeclarationBlock<kw::given>;

pub struct Case {
    kw: kw::case,
    ident: syn::Ident,
    config: Option<Config>,
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
        let config = if case.peek(kw::cfg) {
            Some(case.parse::<Config>()?)
        } else {
            None
        };
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
            config,
            given,
            compute,
            check,
        })
    }
}

impl ToTokens for Case {
    fn to_tokens(&self, tokens: &mut TS) {
        let ident = &self.ident;
        let given = &self.given;
        let compute = &self.compute;
        let config = &self.config;
        let check = &self.check;
        tokens.extend(quote! {
            #[cfg(#config)]
            #[test]
            fn #ident() {
                #given
                #compute
                #check;
            }
        });
    }
}
