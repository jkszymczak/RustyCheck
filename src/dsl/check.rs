use crate::dsl::{expression::Expression, keywords as kw, traits::Code};
use proc_macro2::TokenStream as TS;
use quote::quote;
use syn::{braced, custom_keyword, parse::Parse, Token};

pub struct Check {
    keyword: kw::check,
    conditions: Conditions,
    comment: String
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
        collection: syn::Ident,
        element: syn::Ident,
        condition: Box<Conditions>,
    },
    // TODO: need to work on separation
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
    ForAny,
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
    input.peek(Token![for])&&input.peek2(kw::any)
}

fn parse_for_loop(input: syn::parse::ParseStream,loop_type: LoopType) -> syn::Result<Conditions> {
    input.parse::<Token![for]>()?;
    match loop_type {
        LoopType::ForAny => {input.parse::<kw::any>()?;},
        LoopType::ForEach => {input.parse::<kw::each>()?;},
    }
    let element = input.parse::<syn::Ident>()?;
    input.parse::<Token![in]>()?;
    let collection = input.parse::<syn::Ident>()?;
    input.parse::<Token![,]>()?;
    let conditions = input.parse::<Conditions>()?;
    Ok(Conditions::LoopCondition { collection: collection, element: element, loop_type: loop_type, condition:Box::new(conditions) })
    
}

fn parse_loop_condition(input: syn::parse::ParseStream) -> syn::Result<Conditions> {
    if is_for_each(&input) {
        parse_for_loop(&input,LoopType::ForEach)
    } else if is_in_any(&input) {
        parse_for_loop(&input,LoopType::ForAny)
    } else {
        todo!()
    }
}

impl Parse for Conditions {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {

        if is_for_each(&input) || is_in_any(&input) {
            return parse_loop_condition(input);
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
            Conditions::LoopCondition {loop_type: LoopType::ForEach, collection,element, condition } => {
                let condition = condition.get_code();
                quote!{ #collection.iter().map(| &#element| #condition ).filter(| &#element | #element == false).count() == 0 }
            },
            Conditions::LoopCondition {loop_type: LoopType::ForAny, collection,element, condition } => {
                let condition = condition.get_code();
                quote!{ #collection.iter().map(| &#element| #condition ).filter(| &#element | #element == true).count() != 0 }
            },
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
        let comment = conditions.to_string();
        let conditions = conditions.parse::<Conditions>()?;
        Ok(Check { keyword: kw,comment ,conditions })
        
    }
}
impl Code for Check {
    fn get_code(&self) -> proc_macro2::TokenStream {
        let conditions = self.conditions.get_code();
        let comment = self.comment.clone();
        // TODO: Add custom message to assert where it would show condition with changed values
        quote!{assert!(#conditions,#comment);}
    }
}
