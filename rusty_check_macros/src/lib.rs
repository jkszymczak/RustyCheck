mod dsl;
use dsl::{case::Case, rusty_check::RustyCheck, traits::Code};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
#[proc_macro]
pub fn rusty_check(input: TokenStream) -> TokenStream {
    let rust = parse_macro_input!(input as RustyCheck);
    let output = rust.get_code();
    dbg!(&output);
    quote! {
        // this is macro part
        //#[cfg(test)]
        // mod tests {
            #output
        // }
        //this is not

    }
    .into()
}
