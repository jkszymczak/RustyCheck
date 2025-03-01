use crate::dsl::{expression::Expression, keywords as kw, traits::Code};
use proc_macro2::TokenStream as TS;
use quote::quote;
use syn::{braced, parse::Parse,Token};

pub struct Check {
    keyword: kw::check,
    conditions: Conditions,
}
enum Symbol {
    Equal,
    EqualOr(OtherSymbol),
    Other(OtherSymbol),
}
enum OtherSymbol {
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

struct Condition {
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

impl Parse for JoinType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Ok(_) = input.parse::<kw::and>() {
            Ok(JoinType::And)
        } else {
            input.parse::<kw::or>()?;
            Ok(JoinType::Or)
                
        }
    }
}
impl Code for JoinType {
    fn get_code(&self) -> proc_macro2::TokenStream {
        match self {
            JoinType::Or => quote! {||},
            JoinType::And => quote! {&&},
        }
    }
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
        quote! { #left #symbol #right }
    }
}


fn is_for_each(input: &syn::parse::ParseStream) -> bool {
    input.peek(Token![for]) && input.peek2(kw::each)
}
fn is_in_any(input: &syn::parse::ParseStream) -> bool {
    input.peek(Token![in])&&input.peek2(kw::any)
}

fn parse_loop_condition(input: syn::parse::ParseStream) {
    todo!();
}

impl Parse for Conditions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {

        if is_for_each(&input) || is_in_any(&input) {
            parse_loop_condition(input);
        }
        let condition = input.parse::<Condition>()?;
        if input.peek(kw::or) || input.peek(kw::and){
            let join_type = input.parse::<JoinType>()?;
            return Ok(Conditions::CompoundCondition { left_condition: condition, join: join_type, right_condition: Box::new(input.parse()?)})
        }
        Ok(Conditions::Condition(condition))
    }
}

impl Code for Conditions {
    fn get_code(&self) -> proc_macro2::TokenStream {
        match self {
            Conditions::LoopCondition { loop_type, condition } => todo!(),
            Conditions::CompoundCondition { left_condition, join, right_condition } => {
                let left = left_condition.get_code();
                let join = join.get_code();
                let right = right_condition.get_code();
                quote! { (#left) #join #right }
            },
            Conditions::Condition(condition) => condition.get_code(),
        }
    }
}

impl Parse for Check {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let kw = input.parse::<kw::check>()?;
        let conditions;
        braced!(conditions in input);
        let conditions = conditions.parse::<Conditions>()?;
        Ok(Check { keyword: kw, conditions })
        
    }
}
impl Code for Check {
    fn get_code(&self) -> proc_macro2::TokenStream {
        self.conditions.get_code()
    }
}
