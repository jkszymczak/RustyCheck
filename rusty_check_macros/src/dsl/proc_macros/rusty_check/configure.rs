use super::keywords as kw;
use proc_macro2::{TokenStream as TS, TokenTree};
use quote::{quote, ToTokens};
use syn::{braced, parse::Parse, token::Brace, Ident, Token};
pub struct Config {
    keyword: kw::configure,
    pub elements: TS,
}

impl ToTokens for Config {
    fn to_tokens(&self, tokens: &mut TS) {
        let code = self.elements.clone();
        tokens.extend(quote! {
            #code
        });
    }
}

impl Parse for Config {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let kw = input.parse::<kw::configure>()?;
        _ = input.parse::<Token![=]>()?;
        let mut value_tokens = TS::new();
        while !input.is_empty() {
            let fork = input.fork();
            if fork.peek(Ident) && fork.peek2(Brace) {
                break; // found start of next statement
            }
            let tt: TokenTree = input.parse()?; // consume one token
            value_tokens.extend(std::iter::once(tt));
        }
        Ok(Config {
            keyword: kw,
            elements: value_tokens,
        })
    }
}
