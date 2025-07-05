mod dsl;
use dsl::{
    attribute_macros::automock::{automockfn, TRAIT_REGISTRY},
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
    automockfn(trait_def.clone());
    quote! {
        #[::mockall::automock]
        #trait_def
    }
    .into()
}

#[proc_macro]
pub fn compose_mocks(input: TokenStream) -> TokenStream {
    compose_mocks_fn(input)
}
