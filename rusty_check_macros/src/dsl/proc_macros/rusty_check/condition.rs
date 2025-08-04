use crate::dsl::proc_macros::helpers::{Comment, ToComment};

use super::{super::helpers::get_idents, expression::Expression, keywords as kw};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS;
use quote::{quote, ToTokens};
use syn::{
    parse::{self, Parse},
    parse_macro_input, Expr,
};

/// Represents a logical or comparison operator in a condition.
///
/// Variants:
/// - `Equal`: Represents the `==` operator.
/// - `EqualOr`: Represents `<=` or `>=` depending on the inner [`OtherSymbol`].
/// - `Other`: Represents `<` or `>`.
/// - `Not`: Represents a negation of another [`Symbol`], such as `!=` or logical inversions.
///
/// represents grammar from this diagram:
///
#[doc = include_str!("../../../../../grammar/case/check/condition.svg")]
pub enum Symbol {
    Equal,
    EqualOr(OtherSymbol),
    Other(OtherSymbol),
    Not(Box<Symbol>),
}

/// Represents a non-equality comparison operator.
///
/// Variants:
/// - `Less`: Represents the `<` operator.
/// - `Greater`: Represents the `>` operator.
pub enum OtherSymbol {
    Less,
    Greater,
}

/// Represents a full conditional expression.
///
/// A `Condition` consists of:
/// - `left`: The left-hand side expression.
/// - `symbol`: The operator, represented as a [`Symbol`].
/// - `right`: The right-hand side expression.
pub struct Condition {
    pub left: syn::Expr,
    pub symbol: Symbol,
    pub right: syn::Expr,
}

impl Parse for OtherSymbol {
    /// Parses an `OtherSymbol` from the input stream.
    ///
    /// Recognizes the following patterns:
    /// - `less than` -> `OtherSymbol::Less`
    /// - `greater than` -> `OtherSymbol::Greater`
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
    /// Parses a `Symbol` from the input stream.
    ///
    /// Recognizes the following patterns:
    /// - `equal or less than` -> `Symbol::EqualOr(OtherSymbol::Less)`
    /// - `equal or greater than` -> `Symbol::EqualOr(OtherSymbol::Greater)`
    /// - `equal` -> `Symbol::Equal`
    /// - `not <symbol>` -> `Symbol::Not(Box<Symbol>)`
    /// - `<` or `>` -> `Symbol::Other(OtherSymbol)`
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
    /// Converts an `OtherSymbol` into its token representation.
    ///
    /// - `OtherSymbol::Less` -> `<`
    /// - `OtherSymbol::Greater` -> `>`
    fn to_tokens(&self, tokens: &mut TS) {
        let symbol = match self {
            Self::Less => quote! {<},
            Self::Greater => quote! {>},
        };
        tokens.extend(symbol);
    }
}

impl ToTokens for Symbol {
    /// Converts a `Symbol` into its token representation.
    ///
    /// Handles all variants of `Symbol`, including nested `Not` symbols.
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
    /// Parses a `Condition` from the input stream.
    ///
    /// A `Condition` consists of:
    /// - A left-hand side expression.
    /// - A `Symbol` operator.
    /// - A right-hand side expression.
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let left = input.parse::<syn::Expr>()?;
        let symbol = input.parse::<Symbol>()?;
        let right = input.parse::<syn::Expr>()?;
        dbg!(&right);
        dbg!(&left);
        Ok(Condition {
            left,
            symbol,
            right,
        })
    }
}

impl ToTokens for Condition {
    /// Converts a `Condition` into its token representation.
    ///
    /// Combines the left-hand side, operator, and right-hand side into a single token stream.
    fn to_tokens(&self, tokens: &mut TS) {
        self.left.to_tokens(tokens);
        self.symbol.to_tokens(tokens);
        self.right.to_tokens(tokens);
    }
}

impl ToString for OtherSymbol {
    /// Converts an `OtherSymbol` into a human-readable string.
    ///
    /// - `OtherSymbol::Less` -> `"less than"`
    /// - `OtherSymbol::Greater` -> `"greater than"`
    fn to_string(&self) -> String {
        match self {
            OtherSymbol::Less => "less than".to_owned(),
            OtherSymbol::Greater => "greater than".to_owned(),
        }
    }
}

impl ToString for Symbol {
    /// Converts a `Symbol` into a human-readable string.
    ///
    /// Handles all variants of `Symbol`, including nested `Not` symbols.
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
    /// Converts a `Condition` into a human-readable string.
    ///
    /// Combines the string representations of the left-hand side, operator, and right-hand side.
    fn to_string(&self) -> String {
        self.left.to_token_stream().to_string()
            + " "
            + &self.symbol.to_string()
            + " "
            + &self.right.to_token_stream().to_string()
    }
}

impl ToComment for Condition {
    /// Converts a `Condition` into a `Comment` object.
    ///
    /// The `Comment` includes:
    /// - A string representation of the condition.
    /// - A list of identifiers used in the left-hand and right-hand expressions.
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
