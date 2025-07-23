use super::keywords as kw;
use proc_macro2::TokenStream as TS;
use quote::{quote, ToTokens};
use syn::parse::Parse;

pub struct RustBlock {
    keyword: kw::rust,
    data: TS,
}

pub enum Expression {
    RustExpression(TS),
    RustBlock(RustBlock),
}

impl Parse for Expression {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(kw::rust) {
            let kw = input.parse::<kw::rust>()?;
            let code = input.parse::<syn::ExprBlock>()?;
            let code: TS = quote! { #code }.into();
            let block = RustBlock {
                keyword: kw,
                data: code,
            };
            Ok(Expression::RustBlock(block))
        } else {
            let exp = input.parse::<syn::Expr>()?;
            let exp: TS = quote! {#exp};
            Ok(Expression::RustExpression(exp))
        }
    }
}

impl ToTokens for Expression {
    fn to_tokens(&self, tokens: &mut TS) {
        let code = match self {
            Self::RustExpression(code) => code.clone(),
            Self::RustBlock(block) => block.data.clone(),
        };
        tokens.extend(code);
    }
}
