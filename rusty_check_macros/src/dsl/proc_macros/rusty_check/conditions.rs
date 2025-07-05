use super::{
    super::super::traits::Code, condition::Condition, expression::Expression, keywords as kw,
};
use proc_macro2::TokenStream as TS;
use quote::quote;
use syn::{braced, custom_keyword, parse::Parse, Token};

pub enum Conditions {
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

pub enum LoopType {
    ForAny,
    ForEach,
}
pub enum JoinType {
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

fn is_for_each(input: &syn::parse::ParseStream) -> bool {
    input.peek(Token![for]) && input.peek2(kw::each)
}
fn is_in_any(input: &syn::parse::ParseStream) -> bool {
    input.peek(Token![for]) && input.peek2(kw::any)
}

fn parse_for_loop(input: syn::parse::ParseStream, loop_type: LoopType) -> syn::Result<Conditions> {
    input.parse::<Token![for]>()?;
    match loop_type {
        LoopType::ForAny => {
            input.parse::<kw::any>()?;
        }
        LoopType::ForEach => {
            input.parse::<kw::each>()?;
        }
    }
    let element = input.parse::<syn::Ident>()?;
    input.parse::<Token![in]>()?;
    let collection = input.parse::<syn::Ident>()?;
    input.parse::<Token![,]>()?;
    let conditions = input.parse::<Conditions>()?;
    Ok(Conditions::LoopCondition {
        collection: collection,
        element: element,
        loop_type: loop_type,
        condition: Box::new(conditions),
    })
}

fn parse_loop_condition(input: syn::parse::ParseStream) -> syn::Result<Conditions> {
    if is_for_each(&input) {
        parse_for_loop(&input, LoopType::ForEach)
    } else if is_in_any(&input) {
        parse_for_loop(&input, LoopType::ForAny)
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
        if input.peek(kw::or) || input.peek(kw::and) {
            let join_type = input.parse::<JoinType>()?;
            return Ok(Conditions::CompoundCondition {
                left_condition: condition,
                join: join_type,
                right_condition: Box::new(input.parse()?),
            });
        }
        Ok(Conditions::Condition(condition))
    }
}

impl Code for Conditions {
    fn get_code(&self) -> proc_macro2::TokenStream {
        match self {
            Conditions::LoopCondition {
                loop_type: LoopType::ForEach,
                collection,
                element,
                condition,
            } => {
                let condition = condition.get_code();
                quote! { #collection.iter().map(| &#element| #condition ).filter(| &#element | #element == false).count() == 0 }
            }
            Conditions::LoopCondition {
                loop_type: LoopType::ForAny,
                collection,
                element,
                condition,
            } => {
                let condition = condition.get_code();
                quote! { #collection.iter().map(| &#element| #condition ).filter(| &#element | #element == true).count() != 0 }
            }
            Conditions::CompoundCondition {
                left_condition,
                join,
                right_condition,
            } => {
                let left = left_condition.get_code();
                let join = join.get_code();
                let right = right_condition.get_code();
                quote! { (#left) #join #right }
            }
            Conditions::Condition(condition) => condition.get_code(),
        }
    }
}
