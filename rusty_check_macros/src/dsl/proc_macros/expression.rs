use super::{super::traits::Code, keywords as kw};
use proc_macro2::TokenStream as TS;
use quote::quote;
use syn::{braced, parse::Parse};

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

impl Code for Expression {
    fn get_code(&self) -> proc_macro2::TokenStream {
        match self {
            Self::RustExpression(code) => code.clone(),
            Self::RustBlock(block) => block.data.clone(),
        }
    }
}
