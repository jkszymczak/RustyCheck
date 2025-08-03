use proc_macro2::TokenStream as TS;
use quote::{quote, ToTokens};
use syn::{braced, parse::Parse, Token};

use super::{
    check::Check, compute::Compute, configure::Config, declaration_block::DeclarationBlock,
    keywords as kw,
};

type Given = DeclarationBlock<kw::given>;

/// A struct representing a test case in the RustyCheck DSL.
///
/// This struct contains the following fields:
/// - `kw`: The keyword associated with the case.
/// - `ident`: The identifier for the test case.
/// - `config`: An optional configuration for the test case.
/// - `given`: An optional declaration block for variables used in test case.
/// - `compute`: An optional computation block for the test case.
/// - `check`: The check that will be performed in the test case.
///
/// represents grammar from this diagram:
///
#[doc = include_str!("../../../../../grammar/case/case.svg")]

pub struct Case {
    kw: kw::case,
    ident: syn::Ident,
    config: Option<Config>,
    given: Option<Given>,
    compute: Option<Compute>,
    check: Check,
}

/// Implementation of the `Parse` trait for the `Case` struct.
///
/// This implementation allows parsing a `Case` from a token stream in the RustyCheck DSL.
/// It handles parsing the `case` keyword, identifier, and optional blocks for configuration,
/// variable declarations, computations, and checks.
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

/// Implementation of the `ToTokens` trait for the `Case` struct.
///
/// This implementation converts a `Case` into a token stream that represents a test function
/// in Rust. It includes optional configuration attributes, variable declarations, computations,
/// and the final check.
impl ToTokens for Case {
    fn to_tokens(&self, tokens: &mut TS) {
        let ident = &self.ident;
        let given = &self.given;
        let compute = &self.compute;
        let config = &self.config.as_ref().map(|c| {
            quote! {
                #[cfg(#c)]
            }
        });
        let check = &self.check;
        tokens.extend(quote! {
            #config
            #[test]
            fn #ident() {
                #given
                #compute
                #check;
            }
        });
    }
}
