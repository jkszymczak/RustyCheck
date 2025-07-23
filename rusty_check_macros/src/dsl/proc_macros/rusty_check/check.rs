use super::{conditions::Conditions, keywords as kw};
use proc_macro2::TokenStream as TS;
use quote::{quote, ToTokens};
use syn::{braced, parse::Parse};

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
impl ToTokens for Check {
    fn to_tokens(&self, tokens: &mut TS) {
        let conditions = &self.conditions;
        let comment = &self.comment;
        // TODO: Add custom message to assert where it would show condition with changed values
        tokens.extend(quote! {assert!(#conditions,#comment);});
    }
}
