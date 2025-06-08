mod dsl;
use dsl::{
    attribute_macros::automock::automockfn, proc_macros::rusty_check::RustyCheck, traits::Code,
};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
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
    automockfn(attr, item)
}
