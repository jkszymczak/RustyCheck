use super::{super::traits::Code, keywords as kw};
use proc_macro2::TokenStream as TS;
use quote::quote;
use syn::{braced, custom_keyword, parse::Parse, Token};

pub struct Compute {
    keyword: Token![do],
    rust_code: TS,
}

impl Parse for Compute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let kw = input.parse::<Token![do]>()?;
        let rust_code;
        braced!(rust_code in input);
        Ok(Compute {
            keyword: kw,
            rust_code: rust_code.parse()?,
        })
    }
}
impl Code for Compute {
    fn get_code(&self) -> proc_macro2::TokenStream {
        let code = self.rust_code.clone();
        quote! {#code}.into()
    }
}
