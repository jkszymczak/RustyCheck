use std::marker::PhantomData;

use super::expression::Expression;
use super::{super::super::traits::Code, declaration_block::DeclarationBlock, keywords as kw};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS;
use quote::{quote, ToTokens};
use syn::parse::discouraged::Speculative;
use syn::Error;
use syn::{braced, custom_keyword, parse::Parse, Token};

type Consts = DeclarationBlock<kw::constants>;
type Vars = DeclarationBlock<kw::vars>;
pub struct Global {
    kw: kw::global,
    config: Option<Config>,
    consts: Option<Consts>,
    vars: Option<Vars>,
}

pub struct Config {
    keyword: kw::configure,
}

impl ToTokens for Config {
    fn to_tokens(&self, tokens: &mut TS) {
        todo!()
    }
}

impl Parse for Config {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        todo!()
    }
}

impl Parse for Global {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let kw = input.parse::<kw::global>()?;
        let content;
        braced!(content in input);

        let mut config = None;
        let mut consts = None;
        let mut vars = None;

        while !content.is_empty() {
            if content.peek(kw::configure) {
                let item = content.parse::<Config>()?;
                if config.is_some() {
                    // return Err(syn::Error::new_spanned(item, "Duplicate `configure` block"));
                    todo!()
                }
                config = Some(item);
            } else if content.peek(kw::constants) {
                let item = content.parse::<Consts>()?;
                if consts.is_some() {
                    return Err(syn::Error::new_spanned(item, "Duplicate `constants` block"));
                }
                consts = Some(item);
            } else if content.peek(kw::vars) {
                let item = content.parse::<Vars>()?;
                if vars.is_some() {
                    return Err(syn::Error::new_spanned(item, "Duplicate `vars` block"));
                }
                vars = Some(item);
            } else {
                return Err(content.error("Expected `configure`, `constants`, or `vars` block"));
            }
        }

        Ok(Global {
            kw,
            config,
            consts,
            vars,
        })
    }
}

fn get_tokens_from_option<K: ToTokens>(input: &Option<K>) -> TS {
    match input {
        Some(i) => i.to_token_stream(),
        None => quote! {},
    }
}

impl ToTokens for Global {
    fn to_tokens(&self, tokens: &mut TS) {
        let config = get_tokens_from_option(&self.config);
        let consts = get_tokens_from_option(&self.consts);
        let vars = get_tokens_from_option(&self.vars);
        let code = quote! {
            #consts
            #vars
        };
        tokens.extend(code.into_iter());
    }
}
