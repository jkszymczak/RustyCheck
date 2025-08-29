use super::{case::Case, configure::Config, global::Global, keywords as kw};
use proc_macro2::TokenStream as TS;
use quote::{quote, ToTokens};
use syn::{parse::Parse, Item, Token};

/// Represents a full `rusty_check!` macro input, consisting of:
///
/// 1. Optional global configuration (via [`Global`])  
/// 2. A list of `case` blocks (via [`Case`])  
/// 3. Additional Rust items that will be inserted into the generated test module.
///
/// This structure is parsed directly from the macro input stream using `syn`.
pub struct RustyCheck {
    /// Optional global configuration, starting with the `global` keyword.
    globals: Option<Global>,
    /// A list of `case` blocks that define individual test cases.
    cases: Vec<Case>,
    /// Arbitrary Rust code items to be included in the generated test module.
    rust_code: Vec<Item>,
}

impl RustyCheck {
    fn get_config(&self) -> Config {
        self.globals
            .as_ref()
            .map(|g| g.config.clone())
            .unwrap_or(Config::default())
    }
}

impl Parse for RustyCheck {
    /// Parses the macro input into a [`RustyCheck`] structure.
    ///
    /// Parsing rules:
    /// - An optional `global` block is parsed first if present.
    /// - Subsequent `case` blocks are parsed and pushed into [`Self::cases`].
    /// - Any other Rust items between cases are stored in [`Self::rust_code`].
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut cases = Vec::new();
        let mut rust_code = Vec::new();
        while !input.is_empty() && input.peek(Token![use]) {
            rust_code.push(input.parse::<Item>()?);
        }
        let globals = if input.peek(kw::global) {
            Some(input.parse::<Global>()?)
        } else {
            None
        };

        while !input.is_empty() {
            if input.peek(kw::case) {
                // Parse a test case block
                cases.push(input.parse()?);
            } else {
                // Collect Rust items until we hit the next `case` keyword
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

impl ToTokens for RustyCheck {
    /// Converts the parsed [`RustyCheck`] into a token stream
    /// that generates a `#[cfg(test)]` test module.
    ///
    /// The generated module:
    /// - Is gated by `#[cfg(all(test, <global config>))]`
    /// - Includes any raw Rust items from the macro input
    /// - Expands all global constants and variables
    /// - Expands all test `case` blocks
    fn to_tokens(&self, tokens: &mut TS) {
        let (cfg_flags, consts, vars) = match &self.globals {
            Some(Global {
                config,
                consts,
                vars,
                ..
            }) => (config.get_cfg_flags(), consts, vars),
            None => (TS::new(), &None, &None),
        };
        let config = self.get_config().merge_with_default();
        let cases: &Vec<Case> = &self
            .cases
            .clone()
            .into_iter()
            .map(|c| c.apply_global_config(&config))
            .collect();
        let rust_code = &self.rust_code;
        let module_name = config.get_module_name();
        let create_module = config.get_create_module();
        let body = quote! {
            #(#rust_code)*
            #consts
            #vars
            #(#cases)*
        };

        tokens.extend(match create_module {
            false => body,
            true => quote! {
                #[cfg(all(test, #cfg_flags))]
                mod #module_name {
                    #body
                }
            },
        });
    }
}
