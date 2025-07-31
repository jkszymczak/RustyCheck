use crate::dsl::proc_macros::rusty_check::conditions;

use super::super::helpers::{filter_out_streams_with_ident, Comment, ToComment};
use super::{condition::Condition, expression, keywords as kw};
use proc_macro2::TokenStream as TS;
use quote::{quote, ToTokens};
use syn::{parse::Parse, Token};

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

impl ToTokens for JoinType {
    fn to_tokens(&self, tokens: &mut TS) {
        let join_type = match self {
            JoinType::Or => quote! {||},
            JoinType::And => quote! {&&},
        };
        tokens.extend(join_type);
    }
}

impl ToString for JoinType {
    fn to_string(&self) -> String {
        match self {
            JoinType::Or => "or".to_owned(),
            JoinType::And => "and".to_owned(),
        }
    }
}

impl ToString for LoopType {
    fn to_string(&self) -> String {
        match self {
            LoopType::ForAny => "for any".to_owned(),
            LoopType::ForEach => "for each".to_owned(),
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

impl ToTokens for Conditions {
    fn to_tokens(&self, tokens: &mut TS) {
        let conditions = match self {
            Conditions::LoopCondition {
                loop_type: LoopType::ForEach,
                collection,
                element,
                condition,
            } => {
                quote! { #collection.iter().map(| &#element| #condition ).filter(| &#element | #element == false).count() == 0 }
            }
            Conditions::LoopCondition {
                loop_type: LoopType::ForAny,
                collection,
                element,
                condition,
            } => {
                quote! { #collection.iter().map(| &#element| #condition ).filter(| &#element | #element == true).count() != 0 }
            }
            Conditions::CompoundCondition {
                left_condition,
                join,
                right_condition,
            } => {
                quote! { (#left_condition) #join #right_condition }
            }
            Conditions::Condition(condition) => condition.to_token_stream(),
        };
        tokens.extend(conditions);
    }
}

impl ToString for Conditions {
    fn to_string(&self) -> String {
        match self {
            Conditions::LoopCondition {
                loop_type,
                collection,
                element,
                condition,
            } => {
                loop_type.to_string()
                    + " "
                    + collection.to_string().as_str()
                    + " "
                    + element.to_string().as_str()
                    + ", "
                    + condition.to_string().as_str()
            }
            Conditions::CompoundCondition {
                left_condition,
                join,
                right_condition,
            } => {
                left_condition.to_string()
                    + " "
                    + join.to_string().as_str()
                    + " "
                    + right_condition.to_string().as_str()
            }
            Conditions::Condition(condition) => condition.to_string(),
        }
    }
}

impl ToComment for Conditions {
    fn to_comment(&self) -> Comment {
        match &self {
            Conditions::LoopCondition {
                loop_type: _,
                collection,
                element,
                condition,
            } => {
                let cond_comment = condition.to_comment();
                let comment = self.to_string();
                let filtered_values = filter_out_streams_with_ident(&cond_comment.values, element)
                    .into_iter()
                    .map(|v| v.clone())
                    .collect();
                Comment {
                    string: comment,
                    values: vec![vec![collection.to_token_stream()], filtered_values].concat(),
                }
            }
            Conditions::CompoundCondition {
                left_condition,
                join: _,
                right_condition,
            } => {
                let left_comment = left_condition.to_comment();
                let right_comment = right_condition.to_comment();
                let comment = self.to_string();
                let left_value = left_comment.values;
                let right_value = right_comment.values;
                Comment {
                    string: comment,
                    values: vec![left_value, right_value].concat(),
                }
            }
            Conditions::Condition(condition) => condition.to_comment(),
        }
    }
}
