mod dsl;
use dsl::{
    attribute_macros::mock_registry::{add_to_registry, TRAIT_REGISTRY},
    proc_macros::{
        compose_mocks::compose_mocks::compose_mocks_fn, rusty_check::rusty_check::RustyCheck,
    },
    traits::Code,
    traits::MethodDeclaration,
};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemTrait};
#[proc_macro]
pub fn rusty_check(input: TokenStream) -> TokenStream {
    let rust = parse_macro_input!(input as RustyCheck);
    let output = rust.get_code();
    // dbg!(&output);
    quote! {
            #output
    }
    .into()
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
