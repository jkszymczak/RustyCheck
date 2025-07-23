use proc_macro2::TokenStream as TS;
use quote::{quote, ToTokens};
use syn::{braced, parse::Parse, Token};

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
impl ToTokens for Compute {
    fn to_tokens(&self, tokens: &mut TS) {
        let code = self.rust_code.clone();
        tokens.extend(quote! {#code});
    }
}
