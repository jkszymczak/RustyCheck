use super::{case::Case, global::Global, keywords as kw};
use proc_macro2::TokenStream as TS;
use quote::{quote, ToTokens};
use syn::{parse::Parse, Item};
pub struct RustyCheck {
    globals: Option<Global>,
    cases: Vec<Case>,
    rust_code: Vec<Item>,
}

impl Parse for RustyCheck {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut cases = Vec::new();
        let mut rust_code = Vec::new();
        let globals = if input.peek(kw::global) {
            Some(input.parse::<Global>()?)
        } else {
            None
        };

        while !input.is_empty() {
            if input.peek(kw::case) {
                cases.push(input.parse()?);
            } else {
                while !input.is_empty() && !input.peek(kw::case) {
                    rust_code.push(input.parse::<Item>()?);
                }
            }
        }

        Ok(RustyCheck {
            globals,
            cases,
            rust_code,
        })
    }
}

// fn parse_rust_code_until_case(input: syn::parse::ParseStream) -> syn::Result<TS> {
//     let mut tokens = TS::new();
//     while !input.is_empty() && !input.peek(kw::case) {
//         let tt: proc_macro2::TokenTree = input.parse()?;
//         tokens.extend(std::iter::once(tt));
//     }
//     Ok(tokens)
// }
fn parse_rust_code_until_case(input: syn::parse::ParseStream) -> syn::Result<TS> {
    // parse one top-level Item
    let item: Item = input.parse()?;
    Ok(item.to_token_stream())
}

impl ToTokens for RustyCheck {
    fn to_tokens(&self, tokens: &mut TS) {
        let (config, consts, vars) = match &self.globals {
            Some(Global {
                config,
                consts,
                vars,
                ..
            }) => (config, consts, vars),
            None => (&None, &None, &None),
        };
        let cases = &self.cases;
        let rust_code = &self.rust_code;
        tokens.extend(quote! {
            #[cfg(all(test,#config))]
            mod tests {
                #(#rust_code)*
                #consts
                #vars
                #(#cases)*
            }
        });
    }
}
