use crate::dsl::proc_macros::helpers::{Comment, ToComment};

use super::{super::helpers::get_idents, expression::Expression, keywords as kw};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS;
use quote::{quote, ToTokens};
use syn::{
    parse::{self, Parse},
    parse_macro_input, Expr,
};

pub enum Symbol {
    Equal,
    EqualOr(OtherSymbol),
    Other(OtherSymbol),
    Not(Box<Symbol>),
}
pub enum OtherSymbol {
    Less,
    Greater,
}

pub struct Condition {
    pub left: Expression,
    pub symbol: Symbol,
    pub right: Expression,
}

impl Parse for OtherSymbol {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(kw::less) {
            input.parse::<kw::less>()?;
            input.parse::<kw::than>()?;
            Ok(OtherSymbol::Less)
        } else {
            input.parse::<kw::greater>()?;
            input.parse::<kw::than>()?;
            Ok(OtherSymbol::Greater)
        }
    }
}
impl Parse for Symbol {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(kw::equal) && input.peek2(kw::or) {
            input.parse::<kw::equal>()?;
            input.parse::<kw::or>()?;
            Ok(Symbol::EqualOr(input.parse::<OtherSymbol>()?))
        } else if input.peek(kw::equal) {
            input.parse::<kw::equal>()?;
            Ok(Symbol::Equal)
        } else if input.peek(kw::not) {
            input.parse::<kw::not>()?;
            Ok(Symbol::Not(input.parse()?))
        } else {
            let other = input.parse::<OtherSymbol>()?;
            Ok(Symbol::Other(other))
        }
    }
}

impl ToTokens for OtherSymbol {
    fn to_tokens(&self, tokens: &mut TS) {
        let symbol = match self {
            Self::Less => quote! {<},
            Self::Greater => quote! {>},
        };
        tokens.extend(symbol);
    }
}

impl ToTokens for Symbol {
    fn to_tokens(&self, tokens: &mut TS) {
        let symbol = match self {
            Symbol::Equal => quote! {==},
            Symbol::EqualOr(OtherSymbol::Less) => quote! {<=},
            Symbol::EqualOr(OtherSymbol::Greater) => quote! {>=},
            Symbol::Other(other_symbol) => other_symbol.to_token_stream(),
            Symbol::Not(symbol) => match symbol.as_ref() {
                Symbol::Equal => quote! {!=},
                Symbol::EqualOr(OtherSymbol::Less) => quote! {>},
                Symbol::EqualOr(OtherSymbol::Greater) => quote! {<},
                Symbol::Other(other_symbol) => match other_symbol {
                    OtherSymbol::Less => quote! {>=},
                    OtherSymbol::Greater => quote! {<=},
                },
                Symbol::Not(symbol) => symbol.to_token_stream(),
            },
        };
        tokens.extend(symbol);
    }
}

impl Parse for Condition {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let left = input.parse::<Expression>()?;
        let symbol = input.parse::<Symbol>()?;
        let right = input.parse::<Expression>()?;
        Ok(Condition {
            left,
            symbol,
            right,
        })
    }
}

impl ToTokens for Condition {
    fn to_tokens(&self, tokens: &mut TS) {
        self.left.to_tokens(tokens);
        self.symbol.to_tokens(tokens);
        self.right.to_tokens(tokens);
    }
}

impl ToString for OtherSymbol {
    fn to_string(&self) -> String {
        match self {
            OtherSymbol::Less => "less than".to_owned(),
            OtherSymbol::Greater => "greater than".to_owned(),
        }
    }
}

impl ToString for Symbol {
    fn to_string(&self) -> String {
        match self {
            Symbol::Equal => "equal".to_string(),
            Symbol::EqualOr(OtherSymbol::Less) => "equal or less than".to_owned(),
            Symbol::EqualOr(OtherSymbol::Greater) => "equal or greater than".to_owned(),
            Symbol::Other(other_symbol) => other_symbol.to_string(),
            Symbol::Not(symbol) => "not ".to_owned() + &symbol.to_string(),
        }
    }
}
impl ToString for Condition {
    fn to_string(&self) -> String {
        self.left.to_token_stream().to_string()
            + " "
            + &self.symbol.to_string()
            + " "
            + &self.right.to_token_stream().to_string()
    }
}

impl ToComment for Condition {
    fn to_comment(&self) -> Comment {
        let left = &self.left;
        let right = &self.right;
        let condition_string = self.to_string();
        let left_idents = get_idents(&left.to_token_stream());
        let right_idents = get_idents(&right.to_token_stream());
        let values = vec![left_idents, right_idents]
            .concat()
            .iter()
            .map(|i| i.to_token_stream())
            .collect();
        Comment {
            string: condition_string,
            values: values,
        }
    }
}
