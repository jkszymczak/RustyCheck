mod data;
use data::{dsl::Case, traits::Code};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
#[proc_macro]
pub fn rusty_check(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as Case);
    let declarations = parsed.get_code();
    quote! {
        #declarations
    }
    .into()
}
