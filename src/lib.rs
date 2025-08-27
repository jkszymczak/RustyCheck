mod dsl;
use dsl::proc_macros::rusty_check::rusty_check::RustyCheck;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemTrait};

/// RustyCheck procedural macro that processes the `rusty_check!` DSL.
/// Follows grammar from this diagram:
#[proc_macro]
pub fn rusty_check(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as RustyCheck)
        .to_token_stream()
        .into()
}
