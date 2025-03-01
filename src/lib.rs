mod dsl;
use dsl::{
    check::Check,
    expression::Expression,
    given::{self, Given},
    traits::Code,
};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
#[proc_macro]
pub fn rusty_check(input: TokenStream) -> TokenStream {
    let rust = parse_macro_input!(input as Check);
    let output = rust.get_code();
    dbg!(&output);
    quote! {
        // this is macro part
        assert!(#output);
        //this is not

    }
    .into()
}
