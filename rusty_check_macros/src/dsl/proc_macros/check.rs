use super::{super::traits::Code, conditions::Conditions, expression::Expression, keywords as kw};
use proc_macro2::TokenStream as TS;
use quote::quote;
use syn::{braced, custom_keyword, parse::Parse, Token};

pub struct Check {
    keyword: kw::check,
    conditions: Conditions,
    comment: String,
}

impl Parse for Check {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let kw = input.parse::<kw::check>()?;
        let conditions;
        braced!(conditions in input);
        let comment = conditions.to_string();
        let conditions = conditions.parse::<Conditions>()?;
        Ok(Check {
            keyword: kw,
            comment,
            conditions,
        })
    }
}
impl Code for Check {
    fn get_code(&self) -> proc_macro2::TokenStream {
        let conditions = self.conditions.get_code();
        let comment = self.comment.clone();
        // TODO: Add custom message to assert where it would show condition with changed values
        quote! {assert!(#conditions,#comment);}
    }
}
