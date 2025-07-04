mod dsl;
use dsl::{
    attribute_macros::automock::{automockfn, TRAIT_REGISTRY},
    proc_macros::rusty_check::RustyCheck,
    traits::Code,
    traits::MethodDeclaration,
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

struct ComposeStruct {
    struct_a: syn::Ident,
    struct_b: syn::Ident,
    name: syn::Ident,
}
impl syn::parse::Parse for ComposeStruct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let struct_a = input.parse::<syn::Ident>()?;
        input.parse::<syn::Token![,]>()?;
        let struct_b = input.parse::<syn::Ident>()?;
        input.parse::<syn::Token![,]>()?;
        let name = input.parse::<syn::Ident>()?;
        Ok(ComposeStruct {
            struct_a,
            struct_b,
            name,
        })
    }
}

#[proc_macro_attribute]
pub fn rustymock(attr: TokenStream, item: TokenStream) -> TokenStream {
    automockfn(attr, item)
}

#[proc_macro]
pub fn compose_mocks(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ComposeStruct);
    let struct_a = input.struct_a;
    dbg!(&TRAIT_REGISTRY);
    quote! {}.into()
}
