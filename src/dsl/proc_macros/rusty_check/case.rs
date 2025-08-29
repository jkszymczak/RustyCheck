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
#[derive(Clone)]
pub struct Case {
    ident: syn::Ident,
    config: Config,
    given: Option<Given>,
    compute: Option<Compute>,
    check: Check,
}

impl Case {
    pub fn apply_global_config(self, global_cfg: &Config) -> Case {
        Case {
            config: self.config.merge_with_other_and_default(global_cfg),
            ..self
        }
    }
}
/// Implementation of the `Parse` trait for the `Case` struct.
///
/// This implementation allows parsing a `Case` from a token stream in the RustyCheck DSL.
/// It handles parsing the `case` keyword, identifier, and optional blocks for configuration,
/// variable declarations, computations, and checks.
impl Parse for Case {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        _ = input.parse::<kw::case>()?;
        let ident = input.parse::<syn::Ident>()?;
        let case;
        braced!(case in input);
        let config = if case.peek(kw::cfg) {
            case.parse::<Config>()?
        } else {
            Config::default()
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
        let cfg_flags: TS = self.config.get_cfg_flags();
        let check = self.check.to_owned().set_options(&self.config);
        tokens.extend(quote! {
            #cfg_flags
            #[test]
            fn #ident() {
                #given
                #compute
                #check
            }
        });
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    use quote::ToTokens;
    use syn::{parse_quote, parse_str, Ident};

    #[test]
    fn test_parse_case() {
        let input: proc_macro2::TokenStream = parse_quote! {
            case my_test {
                given {
                    x = 10
                }
                do {
                    let y = x + 5;
                }
                check { x equal y }
            }
        };

        let parsed_case: Case = syn::parse2(input).unwrap();

        assert_eq!(parsed_case.ident.to_string(), "my_test");
        assert!(matches!(parsed_case.given, Some(_)));
        assert!(matches!(parsed_case.compute, Some(_)));
        assert!(matches!(parsed_case.check, Check { .. }));
    }

    #[test]
    fn test_to_tokens_case() {
        let mut tokens = proc_macro2::TokenStream::new();
        let ident: Ident = parse_quote! { my_test_case };
        let given = Some(parse_str("given {x = 20}").unwrap());
        let check = parse_str("check { x equal 20 }").unwrap();
        let case = Case {
            ident,
            config: Config::default(),
            given,
            compute: None,
            check,
        };

        case.to_tokens(&mut tokens);

        assert_eq!(
            tokens.to_string(),
            parse_str::<TS>(r#"#[test] fn my_test_case() { let x = 20; assert!((x == 20),"x equal 20 where, x={:?}",x);}"#).unwrap().to_string()
        );
    }
}
