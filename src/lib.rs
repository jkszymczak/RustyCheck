mod dsl;
use dsl::{
    case::Case,
    traits::Code,
};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
#[proc_macro]
pub fn rusty_check(input: TokenStream) -> TokenStream {
    let rust = parse_macro_input!(input as Case);
    let output = rust.get_code();
    dbg!(&output);
    quote! {
        // this is macro part
        #output
        //this is not

    }
    .into()
}
