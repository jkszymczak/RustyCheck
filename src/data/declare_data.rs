use crate::data::{keywords as kw, traits::Code};
use syn::{braced, parse::Parse, ExprAssign, Token};

use quote::quote;

pub enum DeclareData {
    Given(Given),
    Where(Where),
}

impl Code for DeclareData {
    fn get_code(&self) -> proc_macro2::TokenStream {
        match self {
            DeclareData::Given(data) => data.declarations.clone(),
            DeclareData::Where(data) => data.declarations.clone(),
        }
    }
}

pub struct Given {
    keyword: kw::given,
    pub declarations: proc_macro2::TokenStream,
}

pub struct Where {
    keyword: Token![where],
    pub declarations: proc_macro2::TokenStream,
}

impl Parse for Given {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let kw = input.parse::<kw::given>()?;
        let declarations;
        braced!(declarations in input);
        let parsed_declarations = declarations.parse_terminated(ExprAssign::parse, Token![,])?;
        let declarations = parsed_declarations
            .into_iter()
            .map(|val| {
                quote! {
                    let #val;
                }
            })
            .collect::<proc_macro2::TokenStream>();
        Ok(Given {
            keyword: kw,
            declarations,
        })
    }
}

impl Parse for Where {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let kw = input.parse::<Token![where]>()?;
        let declarations;
        braced!(declarations in input);
        let parsed_declarations = declarations.parse_terminated(ExprAssign::parse, Token![,])?;
        let declarations = parsed_declarations
            .into_iter()
            .map(|val| {
                quote! {
                    let #val;
                }
            })
            .collect::<proc_macro2::TokenStream>();
        Ok(Where {
            keyword: kw,
            declarations,
        })
    }
}

impl Parse for DeclareData {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(kw::given) {
            Ok(DeclareData::Given(input.parse::<Given>()?))
        } else {
            Ok(DeclareData::Where(input.parse::<Where>()?))
        }
    }
}
