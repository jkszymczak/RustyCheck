use crate::dsl::{expression::Expression, keywords as kw, traits::Code};
use proc_macro2::TokenStream as TS;
use quote::quote;
use syn::{braced, parse::Parse};

pub struct Check {
    keyword: kw::check,
    conditions: Vec<Condition>,
}
pub enum Symbol {
    Equal,
    EqualOr(OtherSymbol),
    Other(OtherSymbol),
}
pub enum OtherSymbol {
    Less,
    Greater,
}

enum Conditions {
    LoopCondition {
        loop_type: LoopType,
        condition: Box<Conditions>,
    },
    CompoundCondition {
        left_condition: Condition,
        join: JoinType,
        right_condition: Box<Conditions>,
    },
    Condition(Condition),
}

pub struct Condition {
    left: Expression,
    symbol: Symbol,
    right: Expression,
}

enum LoopType {
    InAny,
    ForEach,
}
enum JoinType {
    Or,
    And,
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
        } else {
            let other = input.parse::<OtherSymbol>()?;
            Ok(Symbol::Other(other))
        }
    }
}

impl Code for OtherSymbol {
    fn get_code(&self) -> proc_macro2::TokenStream {
        match self {
            Self::Less => quote! {<}.into(),
            Self::Greater => quote! {>}.into(),
        }
    }
}

impl Code for Symbol {
    fn get_code(&self) -> proc_macro2::TokenStream {
        match self {
            Symbol::Equal => quote! {==},
            Symbol::EqualOr(OtherSymbol::Less) => quote! {<=},
            Symbol::EqualOr(OtherSymbol::Greater) => quote! {>=},
            Symbol::Other(other_symbol) => other_symbol.get_code(),
        }
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
impl Code for Condition {
    fn get_code(&self) -> proc_macro2::TokenStream {
        let left = self.left.get_code().clone();
        let symbol = self.symbol.get_code().clone();
        let right = self.right.get_code().clone();
        quote! { assert!(#left #symbol #right)}
    }
}
