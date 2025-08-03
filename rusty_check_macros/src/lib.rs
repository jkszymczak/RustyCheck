mod dsl;
use dsl::{
    attribute_macros::mock_registry::add_to_registry,
    proc_macros::{
        compose_mocks::compose_mocks::compose_mocks_fn, rusty_check::rusty_check::RustyCheck,
    },
};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemTrait};

/// RustyCheck procedural macro that processes the `rusty_check!` DSL.
/// Follows grammar from this diagram:
#[doc = include_str!("../../grammar/grammar.svg")]
#[proc_macro]
pub fn rusty_check(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as RustyCheck)
        .to_token_stream()
        .into()
    // dbg!(&output);
}

#[proc_macro_attribute]
pub fn rustymock(attr: TokenStream, item: TokenStream) -> TokenStream {
    let trait_def = parse_macro_input!(item as ItemTrait);
    quote! {
        #[::rusty_check::mocks::append_to_registry]
        #[::mockall::automock]
        #trait_def
    }
    .into()
}

#[proc_macro_attribute]
pub fn append_to_registry(attr: TokenStream, item: TokenStream) -> TokenStream {
    let trait_def = parse_macro_input!(item as ItemTrait);
    add_to_registry(trait_def.clone());
    quote! {
        #trait_def
    }
    .into()
}

#[proc_macro]
pub fn compose_mocks(input: TokenStream) -> TokenStream {
    compose_mocks_fn(input)
}
