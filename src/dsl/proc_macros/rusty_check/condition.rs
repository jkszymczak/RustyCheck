use crate::dsl::proc_macros::helpers::{Comment, ToComment};

use super::{super::helpers::get_idents, configure::CommentType, keywords as kw};
use proc_macro2::TokenStream as TS;
use quote::{quote, ToTokens};
use syn::parse::Parse;

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
#[derive(Debug, PartialEq, Eq, Clone)]
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

#[derive(Debug, PartialEq, Eq, Clone)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
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
        // dbg!(&right);
        // dbg!(&left);
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
        let left = &self.left;
        let symbol = &self.symbol;
        let right = &self.right;
        tokens.extend(quote! {
            ( #left #symbol #right )
        });
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
    fn to_comment(&self, comment_type: CommentType) -> Comment {
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
#[cfg(test)]
mod tests {
    use super::*;
    use syn::{parse_macro_input, parse_str};
    #[test]
    fn test_parse_other_symbol() {
        let input: OtherSymbol = parse_str("less than").unwrap();
        assert_eq!(input, OtherSymbol::Less);
        let input: OtherSymbol = parse_str("greater than").unwrap();
        assert_eq!(input, OtherSymbol::Greater);
    }

    #[test]
    fn test_parse_symbol() {
        let symbol: Symbol = parse_str("equal or less than").unwrap();
        assert_eq!(symbol, Symbol::EqualOr(OtherSymbol::Less));

        let symbol: Symbol = parse_str("equal or greater than").unwrap();
        assert_eq!(symbol, Symbol::EqualOr(OtherSymbol::Greater));

        let symbol: Symbol = parse_str("equal").unwrap();
        assert_eq!(symbol, Symbol::Equal);

        let symbol: Symbol = parse_str("less than").unwrap();
        assert_eq!(symbol, Symbol::Other(OtherSymbol::Less));

        let symbol: Symbol = parse_str("greater than").unwrap();
        assert_eq!(symbol, Symbol::Other(OtherSymbol::Greater));

        let symbol: Symbol = parse_str("not equal").unwrap();
        assert_eq!(symbol, Symbol::Not(Box::new(Symbol::Equal)));
    }

    #[test]
    fn test_parse_condition() {
        let condition: Condition = parse_str("10 less than 20").unwrap();
        assert_eq!(condition.left.to_token_stream().to_string(), "10");
        assert_eq!(condition.symbol, Symbol::Other(OtherSymbol::Less));
        assert_eq!(condition.right.to_token_stream().to_string(), "20");

        let condition: Condition = parse_str("30 equal or greater than 40").unwrap();
        assert_eq!(condition.left.to_token_stream().to_string(), "30");
        assert_eq!(condition.symbol, Symbol::EqualOr(OtherSymbol::Greater));
        assert_eq!(condition.right.to_token_stream().to_string(), "40");

        let condition: Condition = parse_str("50 equal 60").unwrap();
        assert_eq!(condition.left.to_token_stream().to_string(), "50");
        assert_eq!(condition.symbol, Symbol::Equal);
        assert_eq!(condition.right.to_token_stream().to_string(), "60");

        let condition: Condition = parse_str("70 not greater than 80").unwrap();
        assert_eq!(condition.left.to_token_stream().to_string(), "70");
        assert_eq!(
            condition.symbol,
            Symbol::Not(Box::new(Symbol::Other(OtherSymbol::Greater)))
        );
        assert_eq!(condition.right.to_token_stream().to_string(), "80");
    }

    #[test]
    fn test_to_tokens_other_symbol() {
        let other_less = OtherSymbol::Less;
        let token_stream: TS = quote! {#other_less};
        assert_eq!(token_stream.to_string(), "<");

        let other_greater = OtherSymbol::Greater;
        let token_stream: TS = quote! {#other_greater};
        assert_eq!(token_stream.to_string(), ">");
    }

    #[test]
    fn test_to_tokens_symbol() {
        let symbol_equal = Symbol::Equal;
        let token_stream: TS = quote! {#symbol_equal};
        assert_eq!(token_stream.to_string(), "==");

        let symbol_less_or_equal = Symbol::EqualOr(OtherSymbol::Less);
        let token_stream: TS = quote! {#symbol_less_or_equal};
        assert_eq!(token_stream.to_string(), "<=");

        let symbol_greater_or_equal = Symbol::EqualOr(OtherSymbol::Greater);
        let token_stream: TS = quote! {#symbol_greater_or_equal};
        assert_eq!(token_stream.to_string(), ">=");

        let symbol_other_less = Symbol::Other(OtherSymbol::Less);
        let token_stream: TS = quote! {#symbol_other_less};
        assert_eq!(token_stream.to_string(), "<");

        let symbol_other_greater = Symbol::Other(OtherSymbol::Greater);
        let token_stream: TS = quote! {#symbol_other_greater};
        assert_eq!(token_stream.to_string(), ">");

        let symbol_not_equal = Symbol::Not(Box::new(Symbol::Equal));
        let token_stream: TS = quote! {#symbol_not_equal};
        assert_eq!(token_stream.to_string(), "!=");

        let symbol_not_less = Symbol::Not(Box::new(Symbol::Other(OtherSymbol::Less)));
        let token_stream: TS = quote! {#symbol_not_less};
        assert_eq!(token_stream.to_string(), ">=");

        let symbol_not_greater = Symbol::Not(Box::new(Symbol::Other(OtherSymbol::Greater)));
        let token_stream: TS = quote! {#symbol_not_greater};
        assert_eq!(token_stream.to_string(), "<=");
    }

    #[test]
    fn test_to_tokens_condition() {
        let condition_10_less_20 = Condition {
            left: syn::parse_str("10").unwrap(),
            symbol: Symbol::Other(OtherSymbol::Less),
            right: syn::parse_str("20").unwrap(),
        };
        let token_stream: TS = quote! {#condition_10_less_20};
        assert_eq!(token_stream.to_string(), "(10 < 20)");

        let condition_30_greater_equal_40 = Condition {
            left: syn::parse_str("30").unwrap(),
            symbol: Symbol::EqualOr(OtherSymbol::Greater),
            right: syn::parse_str("40").unwrap(),
        };
        let token_stream: TS = quote! {#condition_30_greater_equal_40};
        assert_eq!(token_stream.to_string(), "(30 >= 40)");

        let condition_50_equal_60 = Condition {
            left: syn::parse_str("50").unwrap(),
            symbol: Symbol::Equal,
            right: syn::parse_str("60").unwrap(),
        };
        let token_stream: TS = quote! {#condition_50_equal_60};
        assert_eq!(token_stream.to_string(), "(50 == 60)");

        let condition_not_70_greater_80 = Condition {
            left: syn::parse_str("70").unwrap(),
            symbol: Symbol::Not(Box::new(Symbol::Other(OtherSymbol::Greater))),
            right: syn::parse_str("80").unwrap(),
        };
        let token_stream: TS = quote! {#condition_not_70_greater_80};
        assert_eq!(token_stream.to_string(), "(70 <= 80)");
    }

    #[test]
    fn test_parse_condition_with_function_calls() {
        let condition: Condition = parse_str("foo(10) equal bar(20)").unwrap();
        assert_eq!(
            condition.left.to_token_stream().to_string(),
            str_to_tokens("foo ( 10 )")
        );
        assert_eq!(condition.symbol, Symbol::Equal);
        assert_eq!(
            condition.right.to_token_stream().to_string(),
            str_to_tokens("bar ( 20 )")
        );

        let condition: Condition = parse_str("baz(30) less than qux(40)").unwrap();
        assert_eq!(
            condition.left.to_token_stream().to_string(),
            str_to_tokens("baz ( 30 )")
        );
        assert_eq!(condition.symbol, Symbol::Other(OtherSymbol::Less));
        assert_eq!(
            condition.right.to_token_stream().to_string(),
            str_to_tokens("qux ( 40 )")
        );

        let condition: Condition = parse_str("foo(50) not equal bar(60)").unwrap();
        assert_eq!(
            condition.left.to_token_stream().to_string(),
            str_to_tokens("foo ( 50 )")
        );
        assert_eq!(condition.symbol, Symbol::Not(Box::new(Symbol::Equal)));
        assert_eq!(
            condition.right.to_token_stream().to_string(),
            str_to_tokens("bar ( 60 )")
        );
    }

    #[test]
    fn test_parse_condition_with_enums() {
        let condition: Condition = parse_str("MyEnum::Variant1 equal MyEnum::Variant2").unwrap();
        assert_eq!(
            condition.left.to_token_stream().to_string(),
            str_to_tokens("MyEnum :: Variant1")
        );
        assert_eq!(condition.symbol, Symbol::Equal);
        assert_eq!(
            condition.right.to_token_stream().to_string(),
            str_to_tokens("MyEnum :: Variant2")
        );

        let condition: Condition =
            parse_str("MyEnum::Variant3 not equal MyEnum::Variant4").unwrap();
        assert_eq!(
            condition.left.to_token_stream().to_string(),
            str_to_tokens("MyEnum :: Variant3")
        );
        assert_eq!(condition.symbol, Symbol::Not(Box::new(Symbol::Equal)));
        assert_eq!(
            condition.right.to_token_stream().to_string(),
            str_to_tokens("MyEnum :: Variant4")
        );

        let condition: Condition =
            parse_str("MyEnum::Variant5 less than MyEnum::Variant6").unwrap();
        assert_eq!(
            condition.left.to_token_stream().to_string(),
            str_to_tokens("MyEnum :: Variant5")
        );
        assert_eq!(condition.symbol, Symbol::Other(OtherSymbol::Less));
        assert_eq!(
            condition.right.to_token_stream().to_string(),
            str_to_tokens("MyEnum :: Variant6")
        );
    }

    #[test]
    fn test_parse_condition_with_complex_expressions() {
        let condition: Condition = parse_str("(10 + 20) equal (30 - 40)").unwrap();
        assert_eq!(
            condition.left.to_token_stream().to_string(),
            str_to_tokens("( 10 + 20 )")
        );
        assert_eq!(condition.symbol, Symbol::Equal);
        assert_eq!(
            condition.right.to_token_stream().to_string(),
            str_to_tokens("( 30 - 40 )")
        );

        let condition: Condition = parse_str("foo(10) equal bar(baz(20))").unwrap();
        assert_eq!(
            condition.left.to_token_stream().to_string(),
            str_to_tokens("foo ( 10 )")
        );
        assert_eq!(condition.symbol, Symbol::Equal);
        assert_eq!(
            condition.right.to_token_stream().to_string(),
            str_to_tokens("bar ( baz ( 20 ) )")
        );

        let condition: Condition = parse_str("foo(30) not equal bar(40)").unwrap();
        assert_eq!(
            condition.left.to_token_stream().to_string(),
            str_to_tokens("foo ( 30 )")
        );
        assert_eq!(condition.symbol, Symbol::Not(Box::new(Symbol::Equal)));
        assert_eq!(
            condition.right.to_token_stream().to_string(),
            str_to_tokens("bar ( 40 )")
        );
    }

    #[test]
    fn test_to_tokens_condition_with_function_calls() {
        let condition = Condition {
            left: syn::parse_str("foo(10)").unwrap(),
            symbol: Symbol::Equal,
            right: syn::parse_str("bar(20)").unwrap(),
        };
        let token_stream: TS = quote! {#condition};
        assert_eq!(
            token_stream.to_string(),
            str_to_tokens("( foo ( 10 ) == bar ( 20 ) )")
        );

        let condition = Condition {
            left: syn::parse_str("baz(30)").unwrap(),
            symbol: Symbol::Other(OtherSymbol::Less),
            right: syn::parse_str("qux(40)").unwrap(),
        };
        let token_stream: TS = quote! {#condition};
        assert_eq!(
            token_stream.to_string(),
            str_to_tokens("( baz ( 30 ) < qux ( 40 ) )")
        );

        let condition = Condition {
            left: syn::parse_str("foo(50)").unwrap(),
            symbol: Symbol::Not(Box::new(Symbol::Equal)),
            right: syn::parse_str("bar(60)").unwrap(),
        };
        let token_stream: TS = quote! {#condition};
        assert_eq!(
            token_stream.to_string(),
            str_to_tokens("( foo ( 50 ) != bar ( 60 ) )")
        );
    }

    #[test]
    fn test_to_tokens_condition_with_enums() {
        let condition = Condition {
            left: syn::parse_str("MyEnum::Variant1").unwrap(),
            symbol: Symbol::Equal,
            right: syn::parse_str("MyEnum::Variant2").unwrap(),
        };
        let token_stream: TS = quote! {#condition};
        assert_eq!(
            token_stream.to_string(),
            str_to_tokens("( MyEnum :: Variant1 == MyEnum :: Variant2 )")
        );

        let condition = Condition {
            left: syn::parse_str("MyEnum::Variant3").unwrap(),
            symbol: Symbol::Not(Box::new(Symbol::Equal)),
            right: syn::parse_str("MyEnum::Variant4").unwrap(),
        };
        let token_stream: TS = quote! {#condition};
        assert_eq!(
            token_stream.to_string(),
            str_to_tokens("( MyEnum :: Variant3 != MyEnum :: Variant4 )")
        );

        let condition = Condition {
            left: syn::parse_str("MyEnum::Variant5").unwrap(),
            symbol: Symbol::Other(OtherSymbol::Less),
            right: syn::parse_str("MyEnum::Variant6").unwrap(),
        };
        let token_stream: TS = quote! {#condition};
        assert_eq!(
            token_stream.to_string(),
            str_to_tokens("( MyEnum :: Variant5 < MyEnum :: Variant6 )")
        );
    }

    #[test]
    fn test_to_tokens_condition_with_complex_expressions() {
        let condition = Condition {
            left: syn::parse_str("(10 + 20)").unwrap(),
            symbol: Symbol::Equal,
            right: syn::parse_str("(30 - 40)").unwrap(),
        };
        let token_stream: TS = quote! {#condition};
        assert_eq!(
            token_stream.to_string(),
            str_to_tokens("( ( 10 + 20 ) == ( 30 - 40 ) )")
        );

        let condition = Condition {
            left: syn::parse_str("foo(10)").unwrap(),
            symbol: Symbol::Equal,
            right: syn::parse_str("bar(baz(20))").unwrap(),
        };
        let token_stream: TS = quote! {#condition};
        assert_eq!(
            token_stream.to_string(),
            str_to_tokens("( foo ( 10 ) == bar ( baz ( 20 ) ) )")
        );
    }

    fn str_to_tokens(code: &str) -> String {
        parse_str::<TS>(code).unwrap().to_string()
    }
}
