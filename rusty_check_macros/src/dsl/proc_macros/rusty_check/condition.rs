use super::{expression::Expression, keywords as kw};
use proc_macro2::TokenStream as TS;
use quote::{quote, ToTokens};
use syn::parse::Parse;

enum Symbol {
    Equal,
    EqualOr(OtherSymbol),
    Other(OtherSymbol),
}
enum OtherSymbol {
    Less,
    Greater,
}

pub struct Condition {
    left: Expression,
    symbol: Symbol,
    right: Expression,
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
